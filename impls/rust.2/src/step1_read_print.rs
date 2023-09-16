extern crate rustyline;

use crate::types::MalNode;
use rustyline::{error::ReadlineError, DefaultEditor, Result};
use std::io::{stdout, Write};

mod printer;
mod reader;
mod types;

fn read(s: String) -> MalNode {
    reader::read_str(&s)
}

fn eval(node: MalNode, _env: &str) -> MalNode {
    node
}

fn print(node: MalNode) -> String {
    node.to_string()
}

fn rep(s: String) -> String {
    print(eval(read(s), ""))
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.clone());
                println!("{}", rep(line));
                let _ = stdout().flush();
            }
            Err(ReadlineError::Eof) => break Ok(()),
            Err(ReadlineError::Interrupted) => break Ok(()),

            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
