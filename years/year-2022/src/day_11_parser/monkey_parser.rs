use std::error::Error;
use std::io::BufRead;

use log::debug;
use pest::Parser;

use utils::SantaError;

use crate::day_11_parser::{InspectionOperator, ItemValue, Monkey, MonkeyId, MonkeyItems, Monkeys};

#[derive(pest_derive::Parser)]
#[grammar = "assets/day_11_monkey.pest"]
struct MonkeyParser;

pub fn parse_monkeys<I>(input: I) -> Result<(Monkeys, Vec<MonkeyItems>), Box<dyn Error>>
where
    I: BufRead,
{
    let mut input_lines = input.lines();
    let mut monkeys = Vec::new();
    let mut monkeys_items = Vec::new();

    loop {
        let mut monkey_chunk = String::new();

        // Collect monkey chunks
        for x in 0..7 {
            match input_lines.next() {
                Some(line) => {
                    monkey_chunk.push_str(&line?);
                    monkey_chunk.push('\n');
                }
                None => {
                    if x == 0 {
                        break;
                    } else if x < 6 {
                        return Err(SantaError::InvalidInput(
                            "Missing lines in the input: incomplete monkey.".to_string(),
                        )
                        .into());
                    }
                }
            }
        }

        // End of input
        if monkey_chunk.lines().count() < 6 {
            break;
        }

        let mut monkey_pairs = MonkeyParser::parse(Rule::monkey, &monkey_chunk)?
            .next()
            .unwrap()
            .into_inner();
        // monkey_items > [Item]
        let items_pair = monkey_pairs.next().unwrap().into_inner();
        // monkey_operation > operator
        let operation_pair = monkey_pairs.next().unwrap().into_inner().next().unwrap();
        let inspection_operator = match operation_pair.as_rule() {
            Rule::sum => {
                let modifier: ItemValue = operation_pair
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse()?;
                InspectionOperator::Plus(modifier)
            }
            Rule::multiplication => {
                let modifier: ItemValue = operation_pair
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse()?;
                InspectionOperator::Multiplier(modifier)
            }
            Rule::power => InspectionOperator::Power,
            _ => unreachable!(),
        };

        // monkey_test > test_divisor
        let test_divisor: ItemValue = monkey_pairs
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .parse()?;
        // monkey_true > monkey_id
        let test_true_monkey: MonkeyId =
            monkey_pairs.next().unwrap().into_inner().as_str().parse()?;
        let test_false_monkey: MonkeyId =
            monkey_pairs.next().unwrap().into_inner().as_str().parse()?;

        let new_monkey = Monkey {
            inspection_operator,
            test_divisor,
            test_true_monkey,
            test_false_monkey,
        };

        debug!("New monkey {:?}", new_monkey);

        monkeys.push(new_monkey);

        let mut monkey_items = Vec::new();

        for item in items_pair {
            let new_item: ItemValue = item.as_str().parse()?;
            monkey_items.push(new_item);
        }

        monkeys_items.push(monkey_items);
    }

    Ok((monkeys, monkeys_items))
}
