use std::marker::PhantomData;

use serde::Serializer;

use crate::ports::Input;

#[derive(Debug, serde::Serialize)]
pub struct Solution<I: Input, T> {
    part_01: T,
    part_02: T,
    #[serde(serialize_with = "serialize_input_label")]
    from_input: PhantomData<I>,
}

impl<I: Input, T> Solution<I, T> {
    pub fn new(part_01: T, part_02: T) -> Self {
        Self {
            part_01,
            part_02,
            from_input: PhantomData,
        }
    }

    pub fn part_01(&self) -> &T {
        &self.part_01
    }

    pub fn part_02(&self) -> &T {
        &self.part_02
    }
}

fn serialize_input_label<I, S>(_input: &PhantomData<I>, serializer: S) -> Result<S::Ok, S::Error>
where
    I: Input,
    S: Serializer,
{
    serializer.serialize_str(I::LABEL)
}
