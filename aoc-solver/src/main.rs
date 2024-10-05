use std::path::PathBuf;

use aoc_solver::UniversalSolver;
use aoc_traits::dynamic_solver::Identity;
use clap::Parser;
use parser::parse_year;

#[derive(clap::Parser)]
#[command(version, author, about, arg_required_else_help(true))]
struct Solver {
    #[arg(short, long = "challenge", num_args = 1.., value_parser = parse_year, help = "<year>-<day>-<part>:<input-path>")]
    challenges: Vec<(Identity, PathBuf)>,
}

fn main() {
    let Solver { challenges } = Solver::parse();
    let solver = {
        let mut solver = UniversalSolver::default();
        solver
    };
    let solutions = challenges.into_iter();
}

mod parser {
    use std::path::PathBuf;

    use aoc_traits::{dynamic_solver::Identity, Day, Part, Year};
    use winnow::{ascii::digit1, error::ContextError, seq, token::take_till, Parser};

    pub fn parse_year(input: &str) -> Result<(Identity, PathBuf), clap::Error> {
        let value_err = |err: String| clap::Error::raw(clap::error::ErrorKind::InvalidValue, err);
        let (year, day, part, path): (u16, u8, u8, PathBuf) =
            seq!(digit1::<_, ContextError>.parse_to(), _: '-', digit1.parse_to(), _: '-', digit1.parse_to(), _: ':', take_till(1.., [' ', ]).parse_to())
                .parse(input)
                .map_err(|err| value_err(err.to_string()))?;

        Ok((
            Identity::new(
                Year::try_new(year).map_err(|err| value_err(err.to_string()))?,
                Day::try_new(day).map_err(|err| value_err(err.to_string()))?,
                Part::try_new(part).map_err(|err| value_err(err.to_string()))?,
            ),
            path,
        ))
    }
}
