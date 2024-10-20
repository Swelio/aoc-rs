//! Instructions are there: https://adventofcode.com/2022/day/7

use std::error::Error;
use std::io::{BufReader, Read, Seek};

use utils::{CodeSolution, SantaError};

use crate::day_07_parsers::cmd_parser::parse_cmd;
use crate::day_07_parsers::{FileSize, FileSystem};

pub struct DailySolution;

impl CodeSolution for DailySolution {
    fn run<I>(input: I) -> Result<(), Box<dyn Error>>
    where
        I: Read + Seek,
    {
        let buf_input = BufReader::new(input);
        let filesystem = parse_cmd(buf_input)?;

        let total_size = part_1(&filesystem);
        println!("Total size of directories is: {}", total_size);

        let (best_dir, best_size) = part_2(&filesystem)?;
        println!(
            "Best directory to delete is '{}' with size of {}",
            best_dir, best_size
        );

        Ok(())
    }
}

fn part_1(filesystem: &FileSystem) -> FileSize {
    filesystem
        .iterate_dir_sizes()
        .into_iter()
        .filter_map(|(_, size)| if size <= 100000 { Some(size) } else { None })
        .sum()
}

fn part_2(filesystem: &FileSystem) -> Result<(String, FileSize), SantaError> {
    let unused_space = 70000000
        - filesystem
            .get_size("/")
            .ok_or_else(|| SantaError::InvalidInput("Input has no root.".to_string()))?;
    let missing_space = 30000000 - unused_space;

    filesystem
        .iterate_dir_sizes()
        .into_iter()
        .filter(|(_, size)| *size >= missing_space)
        .min_by_key(|(_, size)| *size)
        .ok_or_else(|| SantaError::InvalidInput("No directory is matching the filter.".to_string()))
}

#[cfg(test)]
mod test_day {
    use std::io::{BufReader, Cursor};
    use utils::setup_log;

    use super::*;

    const SAMPLE_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_file_system() {
        setup_log();

        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let filesystem = parse_cmd(input_cursor).unwrap();

        assert_eq!(filesystem.get_size("/a/e").unwrap(), 584);
        assert_eq!(filesystem.get_size("/a").unwrap(), 94853);
        assert_eq!(filesystem.get_size("/").unwrap(), 48381165);
    }

    #[test]
    fn test_part_1() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let filesystem = parse_cmd(input_cursor).unwrap();
        let total_size = part_1(&filesystem);
        assert_eq!(total_size, 95437);
    }

    #[test]
    fn test_part_2() {
        let input_cursor = BufReader::new(Cursor::new(SAMPLE_INPUT));
        let filesystem = parse_cmd(input_cursor).unwrap();
        let (best_dir, best_size) = part_2(&filesystem).unwrap();
        assert_eq!(best_dir, "/d");
        assert_eq!(best_size, 24933642);
    }
}
