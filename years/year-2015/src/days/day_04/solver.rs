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
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

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
    (1u32..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .map_init(
            || {
                let hasher = Md5::new();
                let digest = [0u8; 16];
                let hash = String::new();

                (hasher, digest, hash)
            },
            |(hasher, digest, hash), suffix| {
                let _ = write!(hasher, "{key}{suffix}");
                hasher.finalize_into_reset(GenericArray::from_mut_slice(digest));
                hash.clear();
                digest
                    .iter()
                    .take((pattern.len() / 2) + 1)
                    .for_each(|byte| {
                        let _ = write!(hash, "{byte:02X}");
                    });

                hash.starts_with(pattern).then_some(suffix)
            },
        )
        .find_first(|suffix| suffix.is_some())
        .flatten()
        .expect("iterator is never empty")
        .to_string()
}
