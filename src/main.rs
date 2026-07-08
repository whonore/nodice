use std::{error::Error, str::FromStr};

use clap::Parser;
use nodice::{expr::Expr, stats::Stats};

#[derive(Parser)]
struct Cli {
    #[arg(value_parser = Expr::from_str)]
    expr: Expr,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Cli { expr } = Cli::parse();

    println!(
        "{expr} (Range: [{}, {}], EV: {}, StdDev: {})",
        expr.min()?,
        expr.max()?,
        expr.expected_value()?,
        expr.std_deviation()?
    );
    if let Ok(simpl) = expr.clone().simplify()
        && simpl != expr
    {
        println!("~> {simpl}");
    }
    println!("=> {}", expr.roll()?);

    Ok(())
}
