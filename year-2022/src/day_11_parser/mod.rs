use utils::SantaError;

pub mod monkey_parser;

pub type Monkeys = Vec<Monkey>;
pub type MonkeyId = usize;
pub type MonkeyItems = Vec<ItemValue>;
pub type ItemValue = u64;

#[derive(Copy, Clone, Debug)]
pub enum InspectionOperator {
    Plus(ItemValue),
    Power,
    Multiplier(ItemValue),
}

impl InspectionOperator {
    pub(crate) fn update_item(self, item: ItemValue) -> ItemValue {
        match self {
            Self::Plus(modifier) => item + modifier,
            Self::Power => item.pow(2),
            Self::Multiplier(modifier) => item * modifier,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Monkey {
    pub(crate) inspection_operator: InspectionOperator,
    pub(crate) test_divisor: ItemValue,
    test_true_monkey: MonkeyId,
    test_false_monkey: MonkeyId,
}

impl Monkey {
    pub fn process_turn<F>(
        self,
        monkey_items: Vec<ItemValue>,
        other_monkeys_items: &mut [Vec<ItemValue>],
        update_item: F,
    ) -> Result<(), SantaError>
    where
        F: Fn(&Monkey, ItemValue) -> ItemValue,
    {
        for item in monkey_items {
            // Inspect item, then get bored
            let item = update_item(&self, item);

            // Test item and determine next monkey
            let next_monkey = if item % self.test_divisor == 0 {
                other_monkeys_items.get_mut(self.test_true_monkey)
            } else {
                other_monkeys_items.get_mut(self.test_false_monkey)
            }
            .ok_or(SantaError::UnknownMonkey)?;

            // Send item to next monkey
            next_monkey.push(item);
        }

        Ok(())
    }
}
