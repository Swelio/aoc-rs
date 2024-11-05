//! Instructions are there: https://adventofcode.com/2022/day/11

use std::cmp::Reverse;
use std::error::Error;
use std::io::{BufReader, Read, Seek};

use utils::CodeSolution;

use crate::day_11_parser::{monkey_parser::parse_monkeys, ItemValue, Monkey, MonkeyItems};

type TotalInspections = u64;

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let (monkeys, monkeys_items) = parse_monkeys(BufReader::new(input))?;

        let monkey_business = part_1(&monkeys, monkeys_items.clone())?;
        println!("Monkey business: {}", monkey_business);

        let monkey_business = part_2(&monkeys, monkeys_items)?;
        println!("Monkey business: {}", monkey_business);

        Ok(())
    }
}

fn process_rounds<F>(
    monkeys: &[Monkey],
    monkeys_items: &mut [MonkeyItems],
    total_rounds: usize,
    update_item: F,
) -> Result<Vec<TotalInspections>, Box<dyn Error>>
where
    F: Fn(&Monkey, ItemValue) -> ItemValue,
{
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..total_rounds {
        for (monkey_id, monkey) in monkeys.iter().enumerate() {
            let monkey_items: Vec<ItemValue> = monkeys_items[monkey_id].drain(..).collect();

            inspections[monkey_id] += monkey_items.len() as TotalInspections;
            monkey.process_turn(monkey_items, monkeys_items, &update_item)?;
        }
    }

    Ok(inspections)
}

fn part_1(
    monkeys: &[Monkey],
    mut monkeys_items: Vec<Vec<ItemValue>>,
) -> Result<TotalInspections, Box<dyn Error>> {
    let mut inspections = process_rounds(monkeys, &mut monkeys_items, 20, |monkey, item| {
        (monkey.inspection_operator.update_item(item) as f32 / 3.0).floor() as ItemValue
    })?;
    // Retrieve top 2 inspections and multiply them by each other
    inspections.sort_by_key(|x| Reverse(*x));

    let total_business = inspections[0] as TotalInspections * inspections[1] as TotalInspections;

    Ok(total_business)
}

fn part_2(
    monkeys: &[Monkey],
    mut monkeys_items: Vec<Vec<ItemValue>>,
) -> Result<TotalInspections, Box<dyn Error>> {
    let supermodulo: ItemValue = monkeys.iter().map(|x| x.test_divisor).product();
    println!("Got a supermodulo of: {}", supermodulo);
    let mut inspections = process_rounds(monkeys, &mut monkeys_items, 10000, |monkey, item| {
        monkey.inspection_operator.update_item(item) % supermodulo
    })?;
    // Retrieve top 2 inspections and multiply them by each other
    inspections.sort_by_key(|x| Reverse(*x));

    let total_business = inspections[0] as TotalInspections * inspections[1] as TotalInspections;

    Ok(total_business)
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};

    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../assets/day_11_test_input.txt");

    #[test]
    fn test_monkey_parsing() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let (monkeys, monkeys_items) = parse_monkeys(input_cursor).unwrap();

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys_items[0], [79, 98]);
        assert_eq!(monkeys_items[1], [54, 65, 75, 74]);
        assert_eq!(monkeys_items[2], [79, 60, 97]);
        assert_eq!(monkeys_items[3], [74]);
    }

    #[test]
    fn test_part_1() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let (monkeys, monkeys_items) = parse_monkeys(input_cursor).unwrap();
        let monkey_business = part_1(&monkeys, monkeys_items).unwrap();

        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let (monkeys, monkeys_items) = parse_monkeys(input_cursor).unwrap();
        let monkey_business = part_2(&monkeys, monkeys_items).unwrap();

        assert_eq!(monkey_business, 2713310158);
    }
}
