use std::str::FromStr;

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, usize},
    error::Error,
    sequence::delimited,
};

use crate::expr::Expr;

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

fn parse(s: &str) -> IResult<&str, Expr> {
    delimited(multispace0, expr, multispace0).parse_complete(s)
}

fn expr(s: &str) -> IResult<&str, Expr> {
    alt((repeat, die)).parse(s)
}

fn repeat(s: &str) -> IResult<&str, Expr> {
    let (s, n) = usize(s)?;
    let (s, expr) = expr(s)?;
    Ok((s, expr.repeat(n)))
}

fn die(s: &str) -> IResult<&str, Expr> {
    let (s, _) = tag("d")(s)?;
    let (s, sides) = usize(s)?;
    Ok((s, Expr::die(sides)))
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

    #[test]
    fn bare_die_ok() {
        parse_ok!("d6", Expr::die(6));
    }

    #[test]
    fn repeat_die_ok() {
        parse_ok!("1d6", Expr::die(6).repeat(1));
        parse_ok!("2d6", Expr::die(6).repeat(2));
    }

    #[test]
    fn ws_ok() {
        parse_ok!(" \t2d6", Expr::die(6).repeat(2));
        parse_ok!("2d6\n ", Expr::die(6).repeat(2));
        parse_ok!(" \n2d6\t ", Expr::die(6).repeat(2));
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
    fn invalid_char() {
        assert!("x".parse::<Expr>().is_err());
    }
}
