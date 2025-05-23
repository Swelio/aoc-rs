use crate::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
};

pub trait Challenge<S> {
    fn solve(&self) -> SantaResult<S>;
}

#[diagnostic::do_not_recommend]
impl<T: ?Sized> Challenge<(Solution<Part1>, Solution<Part2>)> for T
where
    T: Challenge<Solution<Part1>> + Challenge<Solution<Part2>>,
{
    fn solve(&self) -> SantaResult<(Solution<Part1>, Solution<Part2>)> {
        let part_1 = self.solve()?;
        let part_2 = self.solve()?;

        Ok((part_1, part_2))
    }
}
