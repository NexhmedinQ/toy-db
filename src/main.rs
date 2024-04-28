use pest::error::Error;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use crate::cursor::execute_sql;
use crate::parser::{Command, parse_sql, Rule};

mod parser;
mod cursor;

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                let command = parse_sql(&line).unwrap();
                println!("{:?}", command);
                let response = execute_sql(command);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
   Ok(())
}
