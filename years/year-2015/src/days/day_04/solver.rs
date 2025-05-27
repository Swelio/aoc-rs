use std::fmt::Write as FmtWrite;
use std::io::Write as IOWrite;

use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};
use md5::{Digest, Md5, digest::generic_array::GenericArray};

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
    let mut hasher = Md5::new();
    let mut digest = [0u8; 16];
    let mut hash = String::new();

    (1u64..)
        .find(|suffix| {
            let _ = write!(&mut hasher, "{key}{suffix}");
            hasher.finalize_into_reset(GenericArray::from_mut_slice(&mut digest));
            hash.clear();
            digest
                .iter()
                .take((pattern.len() / 2) + 1)
                .for_each(|byte| {
                    let _ = write!(&mut hash, "{byte:02X}");
                });

            hash.starts_with(pattern)
        })
        .expect("iterator is never empty")
        .to_string()
}
