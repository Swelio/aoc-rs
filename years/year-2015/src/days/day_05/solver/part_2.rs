use std::collections::HashMap;

use winnow::{
    Parser,
    combinator::{peek, preceded, repeat, seq, terminated},
    stream::AsChar,
    token::{any, take},
};

pub fn parse_valid_pairs<'input>(input: &'input str) -> Vec<&'input str> {
    let parser = |input: &mut &'input str| -> winnow::Result<Option<&'input str>> {
        if input.len() < 2 {
            take(input.len().max(1usize)).void().parse_next(input)?;
            return Ok(None);
        }

        if input.len() == 2 {
            return Ok(Some(take(2usize).parse_next(input)?));
        }

        let pair = terminated(peek(take(2usize)), any).parse_next(input)?;
        let is_repeated = peek::<_, _, (), _>(preceded(any, pair))
            .parse_next(input)
            .is_ok();
        let is_overlapping = peek::<_, _, (), _>(pair).parse_next(input).is_ok();

        if is_repeated {
            return Ok(Some(pair));
        }

        if is_overlapping {
            take(1usize).void().parse_next(input)?;
        }

        Ok(Some(pair))
    };

    let pairs = repeat(0.., parser)
        .map(|pairs: Vec<Option<&'input str>>| {
            pairs
                .into_iter()
                .flatten()
                .fold(HashMap::new(), |mut acc, current| {
                    acc.entry(current)
                        .and_modify(|e| {
                            *e += 1;
                        })
                        .or_insert(1usize);
                    acc
                })
        })
        .parse(input)
        .expect("input already verified through parsing");

    pairs
        .into_iter()
        .filter_map(|(pair, count)| (count >= 2).then_some(pair))
        .collect()
}

pub fn parse_repeated_letter<'input>(input: &'input str) -> Vec<char> {
    let parser = |input: &mut &'input str| -> winnow::Result<Option<_>> {
        if input.len() < 3 {
            take(input.len().max(1usize)).void().parse_next(input)?;
            return Ok(None);
        }

        let mut parse_letter = |input: &mut &str| -> winnow::Result<_> {
            any.verify(|c: &char| c.is_alpha()).parse_next(input)
        };
        let parse_slice = seq!(parse_letter, _: any, parse_letter);
        let (first, second) = terminated(peek(parse_slice), any).parse_next(input)?;

        Ok((first == second).then_some(first))
    };

    repeat(0.., parser)
        .map(|letters: Vec<_>| letters.into_iter().flatten().collect::<Vec<_>>())
        .parse(input)
        .expect("input already verified through parsing")
}
