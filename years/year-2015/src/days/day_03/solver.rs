use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};

use crate::days::day_03::components::{EastWest, Position, SouthNorth};

use super::{
    components::{Coordinates, HousesCollection, NextMove},
    input::Input,
};

impl Challenge<Solution<Part1>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        let (houses, _) = self.into_iter().copied().fold(
            (HousesCollection::new(1), Coordinates::default()),
            |(mut houses, position), current| {
                let position = match current {
                    NextMove::North => position + Position::<SouthNorth>::new(1),
                    NextMove::South => position + Position::<SouthNorth>::new(-1),
                    NextMove::East => position + Position::<EastWest>::new(1),
                    NextMove::West => position + Position::<EastWest>::new(-1),
                };

                houses
                    .as_mut()
                    .entry(position)
                    .and_modify(|presents| *presents += 1)
                    .or_insert(1);

                (houses, position)
            },
        );

        Solution::try_new(houses.as_ref().len())
    }
}

impl Challenge<Solution<Part2>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part2>> {
        let (mut santa, mut bot) = (Coordinates::default(), Coordinates::default());
        let houses = self.into_iter().copied().enumerate().fold(
            HousesCollection::new(1),
            |mut houses, (index, current)| {
                let position = if index % 2 == 0 { &mut santa } else { &mut bot };

                *position = match current {
                    NextMove::North => *position + Position::<SouthNorth>::new(1),
                    NextMove::South => *position + Position::<SouthNorth>::new(-1),
                    NextMove::East => *position + Position::<EastWest>::new(1),
                    NextMove::West => *position + Position::<EastWest>::new(-1),
                };

                houses
                    .as_mut()
                    .entry(*position)
                    .and_modify(|presents| *presents += 1)
                    .or_insert(1);

                houses
            },
        );

        Solution::try_new(houses.as_ref().len())
    }
}
