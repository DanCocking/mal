extern crate rustyline;

use rustyline::{DefaultEditor, Result, error::ReadlineError};

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
} 

fn print(s: String) -> String {
    println!("{}", s);
    s
}

fn rep(s: String) -> String {
    let mut read_str = read(s);
    read_str = eval(read_str);
    print(read_str)
}



fn main() -> Result<()>  {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.clone());
                rep(line);
            }
            Err(ReadlineError::Eof) => {
                break Ok(())
            } 
            Err(ReadlineError::Interrupted) => {
                break Ok(())
            }

            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
        
    }
}
