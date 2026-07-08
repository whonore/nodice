use std::str::FromStr;

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, multispace0, u32},
    combinator::{all_consuming, fail, opt},
    error::{Error as NomError, FromExternalError, ParseError},
    sequence::{delimited, preceded},
};
use nom_language::precedence::{Assoc, Operation, binary_op, precedence};

use crate::{
    error::Error,
    expr::{Expr, binop::Op},
};

impl FromStr for Expr {
    type Err = NomError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_rem, expr)) => Ok(expr),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

// Expr ::= Num Expr | Num | d Num | ( Expr ) | Expr Op Expr
// Num ::= [0-9]+
// Op ::= + | -
fn parse(s: &str) -> IResult<&str, Expr> {
    all_consuming(ws(expr)).parse(s)
}

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

fn expr(s: &str) -> IResult<&str, Expr> {
    let operand = alt(((opt_repeat(alt((die, parens)))), scalar));
    precedence(
        fail(),
        fail(),
        binary_op(2, Assoc::Left, op),
        operand,
        |op: Operation<(), (), Op, Expr>| match op {
            Operation::Binary(lhs, op, rhs) => Ok(op.apply(lhs, rhs)),
            Operation::Prefix(..) | Operation::Postfix(..) => Err("No prefix or postfix"),
        },
    )(s)
}

pub fn opt_repeat<'a, E, F>(inner: F) -> impl Parser<&'a str, Output = Expr, Error = E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, Error>,
    F: Parser<&'a str, Output = Expr, Error = E>,
{
    (opt(u32), inner).map_res(|(repeat, expr)| {
        if let Some(repeat) = repeat {
            expr.repeat(repeat)
        } else {
            Ok(expr)
        }
    })
}

fn scalar(s: &str) -> IResult<&str, Expr> {
    i32.map(Expr::scalar).parse(s)
}

fn die(s: &str) -> IResult<&str, Expr> {
    preceded(tag("d"), u32).map(Expr::die).parse(s)
}

fn parens(s: &str) -> IResult<&str, Expr> {
    delimited(ws(char('(')), expr, ws(char(')'))).parse(s)
}

fn op(s: &str) -> IResult<&str, Op> {
    ws(alt((
        char('+').map(|_| Op::Add),
        char('-').map(|_| Op::Sub),
    )))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;

    macro_rules! parse_ok {
        ($s:expr, $expected:expr) => {
            match $s.parse::<Expr>() {
                Ok(got) => {
                    assert_eq!(got, $expected.into(), "{}", $s);
                }
                Err(err) => {
                    panic!("{}: {err}", $s)
                }
            }
        };
    }

    fn d(n: u32) -> Expr {
        Expr::die(n)
    }

    fn r(e: impl Into<Expr>, n: u32) -> Expr {
        e.into().repeat(n).unwrap()
    }

    #[test]
    fn scalar_ok() {
        parse_ok!("6", 6);
    }

    #[test]
    fn bare_die_ok() {
        parse_ok!("d6", d(6));
    }

    #[test]
    fn repeat_die_ok() {
        parse_ok!("1d6", r(d(6), 1));
        parse_ok!("2d6", r(d(6), 2));
    }

    #[test]
    fn binop_ok() {
        parse_ok!("1d6 + 2d4", r(d(6), 1) + r(d(4), 2));
        parse_ok!("2d6 - d2", r(d(6), 2) - d(2));
        parse_ok!("2d6 - 2", r(d(6), 2) - 2.into());
    }

    #[test]
    fn binop_left_assoc() {
        parse_ok!("d1 - d2 + d3", (d(1) - d(2)) + d(3));
        parse_ok!("(d1 - d2) + d3", (d(1) - d(2)) + d(3));
        parse_ok!("d1 - (d2 + d3)", d(1) - (d(2) + d(3)));
    }

    #[test]
    fn parens_ok() {
        parse_ok!("(d6)", d(6));
        parse_ok!("((d6))", d(6));
        parse_ok!("(1d6)", r(d(6), 1));
        parse_ok!("(1d6 + 2d4)", r(d(6), 1) + r(d(4), 2));
        parse_ok!("(1d6 + 2d4) - d8", r(d(6), 1) + r(d(4), 2) - d(8));
        parse_ok!("1d6 + (2d4 - d8)", r(d(6), 1) + (r(d(4), 2) - d(8)));
    }

    #[test]
    fn repeat_parens_ok() {
        parse_ok!("2(6)", r(6, 2));
        parse_ok!("2(d6)", r(d(6), 2));
        parse_ok!("2((d6))", r(d(6), 2));
        parse_ok!("2(3d6)", r(d(6), 6));
        parse_ok!("2(3(4d6))", r(d(6), 24));
        parse_ok!("2(1d6 + 2d4)", r(r(d(6), 1) + r(d(4), 2), 2));
        parse_ok!("2(1d6 + 2d4) - d8", r(r(d(6), 1) + r(d(4), 2), 2) - d(8));
        parse_ok!(
            "2(1d6 + 3(2d4 - d8))",
            r(r(d(6), 1) + r(r(d(4), 2) - d(8), 3), 2)
        );
    }

    #[test]
    fn ws_ok() {
        parse_ok!(" \t2d6", r(d(6), 2));
        parse_ok!("2d6\n ", r(d(6), 2));
        parse_ok!(" \n2d6\t ", r(d(6), 2));
        parse_ok!(" \n(  2d6\t) ", r(d(6), 2));
        parse_ok!(" \n(  2d6\t-d4)\n+(\nd8 ) ", r(d(6), 2) - d(4) + d(8));
    }

    #[test]
    fn empty_err() {
        assert!("".parse::<Expr>().is_err());
    }

    #[test]
    fn missing_sides_err() {
        assert!("d".parse::<Expr>().is_err());
        assert!("1d".parse::<Expr>().is_err());
    }

    #[test]
    fn space_before_sides_err() {
        assert!("d 6".parse::<Expr>().is_err());
    }

    #[test]
    fn space_after_repeat_err() {
        assert!("1 d6".parse::<Expr>().is_err());
    }

    #[test]
    fn invalid_char() {
        assert!("x".parse::<Expr>().is_err());
    }
}
