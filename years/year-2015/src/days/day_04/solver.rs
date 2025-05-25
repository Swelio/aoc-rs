use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};
use md5::{Digest, Md5};

use super::Input;

impl Challenge<Solution<Part1>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        let solution = bruteforce(self.as_ref(), "00000");
        Solution::try_new(solution)
    }
}

impl Challenge<Solution<Part2>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part2>> {
        {
            let solution = bruteforce(self.as_ref(), "000000");
            Solution::try_new(solution)
        }
    }
}

fn bruteforce(key: &str, pattern: &str) -> String {
    let hasher = Md5::new_with_prefix(key);

    (1u64..)
        .find_map(|suffix| {
            let mut hasher = hasher.clone();
            let suffix = suffix.to_string();
            hasher.update(&suffix);
            let hash = hasher
                .finalize()
                .into_iter()
                .map(|byte| format!("{byte:02X}"))
                .take(pattern.len())
                .collect::<String>();

            (hash.starts_with(pattern)).then_some(suffix)
        })
        .expect("iterator is never empty")
}
