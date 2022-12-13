use std::error::Error;
use std::io::BufRead;

use log::debug;
use pest::Parser;

use crate::day_07_parsers::FileSystem;

#[derive(pest_derive::Parser)]
#[grammar = "assets/day_07_cmd.pest"]
struct CmdParser;

pub fn parse_cmd<I>(mut input: I) -> Result<FileSystem, Box<dyn Error>>
where
    I: BufRead,
{
    let mut filesystem = FileSystem::new();
    let mut current_path = Vec::new();

    loop {
        let mut cmd_input = Vec::new();

        if input.read_until(b'$', &mut cmd_input)? == 0 {
            break;
        }

        let cmd_input = String::from_utf8(cmd_input)?
            .trim_matches('$')
            .trim()
            .to_string();

        // First commandline
        if cmd_input.is_empty() {
            continue;
        }

        debug!("Parsing commandline: '{}'", cmd_input);

        let mut cmd_pairs = CmdParser::parse(Rule::command_process, &cmd_input)?
            .next()
            .unwrap()
            .into_inner();
        let mut cmd_input = cmd_pairs.next().unwrap().into_inner();
        let cmd = cmd_input.next().unwrap();
        let args = cmd_input.next().map(|x| x.as_str());

        match cmd.as_str() {
            "cd" => {
                let next_path = args.unwrap_or("/").trim();

                match next_path {
                    "/" => {
                        current_path.clear();
                        current_path.push("/".to_string());
                    }
                    ".." => {
                        current_path.pop();
                    }
                    any => {
                        current_path.push(any.to_owned());
                    }
                }
            }
            "ls" => {
                // Get ls_output pairs
                let ls_output = cmd_pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner();

                debug!(
                    "Listing files of {}",
                    current_path.join("/").replace("//", "/")
                );

                for listed_path in ls_output {
                    let path_pairs = listed_path.into_inner().next().unwrap();

                    match path_pairs.as_rule() {
                        Rule::file_path => {
                            let mut file_pair = path_pairs.into_inner();
                            let file_size = file_pair.next().unwrap().as_str().parse()?;
                            let file_path = file_pair.next().unwrap();

                            current_path.push(file_path.as_str().to_string());
                            debug!("New file {}", current_path.join("/").replace("//", "/"));
                            filesystem.new_file(
                                current_path.join("/").replace("//", "/").as_str(),
                                file_size,
                            );
                            current_path.pop();
                        }
                        Rule::dir_path => {
                            let dir_path = path_pairs.into_inner().next().unwrap();

                            current_path.push(dir_path.as_str().to_string());
                            debug!(
                                "New directory {}",
                                current_path.join("/").replace("//", "/")
                            );
                            filesystem.new_dir(current_path.join("/").replace("//", "/").as_str());
                            current_path.pop();
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(filesystem)
}
