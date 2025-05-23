use std::{collections::HashMap, marker::PhantomData, ops::Add};

#[derive(Debug, Clone, Copy)]
pub enum NextMove {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, derive_more::AsRef, derive_more::AsMut)]
pub struct HousesCollection(HashMap<Coordinates, usize>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, derive_more::Constructor)]
pub struct Coordinates(Position<SouthNorth>, Position<EastWest>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position<A>(PhantomData<A>, i32);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SouthNorth;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EastWest;

impl<A> Position<A> {
    pub fn new(value: i32) -> Self {
        Self(PhantomData, value)
    }
}

impl HousesCollection {
    pub fn new(start: usize) -> Self {
        Self(HashMap::from([(Coordinates::default(), start)]))
    }
}

impl Add<Position<SouthNorth>> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Position<SouthNorth>) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl Add<Position<EastWest>> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Position<EastWest>) -> Self::Output {
        Self(self.0, self.1 + rhs)
    }
}

impl<A> Add for Position<A> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(PhantomData, self.1 + rhs.1)
    }
}
