// Copyright (C) 2026 wgrav
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{Parser, Subcommand};
use rustyline::{DefaultEditor, error::ReadlineError};
use std::{fs::File, io::Read, path::PathBuf};

use crate::scanner::scan_tokens;
mod errors;
mod scanner;
mod tokens;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Tokenize {
        #[arg()]
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(cmd) = args.cmd {
        match cmd {
            Commands::Tokenize { file } => tokenize_file(file),
        }
    } else {
        run_prompt()
    }
}

fn tokenize_file(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens = scan_tokens(&contents);

    println!("{:?}", tokens);

    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let read = rl.readline("> ");
        match read {
            Ok(line) => {
                let tokens = scan_tokens(&line);
                println!("{:?}", tokens);
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Caught ^D, exiting.");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
