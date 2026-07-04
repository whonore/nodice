use std::str::FromStr;

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, u32},
    combinator::{all_consuming, opt},
    error::{Error, ParseError},
    sequence::{delimited, preceded},
};

use crate::expr::{Expr, binop::Op};

impl FromStr for Expr {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_rem, expr)) => Ok(expr),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

// Expr ::= ExprLhs ExprRhs
// ExprLhs ::= Num Expr | d Num | ( Expr )
// ExprRhs ::= Op Expr | ""
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
    (expr_lhs, opt(expr_rhs))
        .map(|(lhs, rhs)| {
            if let Some((op, rhs)) = rhs {
                op.apply(lhs, rhs)
            } else {
                lhs
            }
        })
        .parse(s)
}

fn expr_lhs(s: &str) -> IResult<&str, Expr> {
    alt((repeat, die, parens)).parse(s)
}

fn expr_rhs(s: &str) -> IResult<&str, (Op, Expr)> {
    (op, expr).parse(s)
}

fn repeat(s: &str) -> IResult<&str, Expr> {
    (u32, expr_lhs)
        .map_res(|(repeat, expr)| expr.repeat(repeat))
        .parse(s)
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
                    assert_eq!(got, $expected, "{}", $s);
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

    fn r(e: Expr, n: u32) -> Expr {
        e.repeat(n).unwrap()
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
