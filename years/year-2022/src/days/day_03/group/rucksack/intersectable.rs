use std::collections::HashSet;

use super::compartment::Compartment;

pub trait Intersectable<'a>
where
    Self: Iterator<Item = &'a Compartment>,
{
    fn intersect(&mut self) -> Compartment;
}

impl<'a, T> Intersectable<'a> for T
where
    T: Iterator<Item = &'a Compartment>,
{
    fn intersect(&mut self) -> Compartment {
        let init = match self.next() {
            None => return Compartment::new(HashSet::new()),
            Some(init) => init.to_owned(),
        };

        self.fold(init, |intersection, current| {
            intersection.intersection(current)
        })
    }
}
