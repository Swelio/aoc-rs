use nutype::nutype;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::{alt, delimited, separated, terminated},
    token::any,
    PResult, Parser,
};

pub type Cargo = Vec<Stack>;
pub type Stack = Vec<Crate>;

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, AsRef))]
pub struct Crate(char);

pub fn parse_cargo(input: &mut &str) -> PResult<Cargo> {
    let line_parser = separated(1.., parse_crate, ' ').map(|line: Vec<Option<Crate>>| line);
    let lines_parser =
        separated(1.., line_parser, line_ending).map(|lines: Vec<Vec<Option<Crate>>>| lines);

    let stacks_parser = separated(1.., delimited(' ', digit1, ' '), ' ').map(|_: ()| ());
    let mut lines_parser = terminated(lines_parser, (line_ending, stacks_parser));

    let stack_lines: Vec<Vec<Option<Crate>>> = lines_parser.parse_next(input)?;
    let stacks = stack_lines
        .into_iter()
        .fold(Vec::new(), |mut stacks: Vec<Vec<Crate>>, line| {
            for (index, item) in line.into_iter().enumerate() {
                let item = match item {
                    Some(item) => item,
                    None => return stacks,
                };
                match stacks.get_mut(index) {
                    Some(stack) => stack.push(item),
                    None => {
                        stacks.push(vec![item]);
                    }
                }
            }

            stacks
        });

    Ok(stacks)
}

fn parse_crate(input: &mut &str) -> PResult<Option<Crate>> {
    let some_parser = delimited(
        '[',
        any.verify(|token: &char| token.is_ascii_uppercase()),
        ']',
    )
    .map(Some);
    let none_parser = "   ".map(|_| None);
    let mut parser = alt((some_parser, none_parser));
    let inner = parser.parse_next(input)?;

    Ok(inner.map(Crate::new))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("cargo.txt");

    #[test]
    fn parsing_cargo() {
        let _ = parse_cargo
            .parse(INPUT)
            .map_err(|err| {
                eprintln!("{err}");
                err
            })
            .unwrap();
    }
}
