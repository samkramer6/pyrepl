use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::py_evaluate::evaluate_code;

mod repl_setup;
mod py_evaluate;

#[allow(dead_code)]
#[allow(unused_imports)]

fn main() {

    repl_setup::welcome_message();    
    let mut entire_string = String::new().to_owned();

    let mut editor = DefaultEditor::new();
    
    'outer: loop {
           
        // Rustyline read
            let input = editor.as_mut().expect("error").readline("PyREPL> ");
        
        // Handle the error
            match input {

                // Handle Ok statements
                Ok(okay_input) => {

                    // Format to string
                        let mut input_string = okay_input.to_owned();
                        
                    // Parse inputs
                        match input_string.trim() {
                            "exit" => break 'outer,
                            "quit" => break 'outer,
                            "clear" => entire_string.replace_range(.., ""),
                            "cls" => clearscreen::clear().unwrap(),
                            "workspace" => println!("{}", entire_string),
                            _ => {
                                input_string.push('\n');
                                let _entire_string = evaluate_code(&mut entire_string, input_string);
                            }
                        }
                },
                
                // Error Handlers  
                Err(ReadlineError::Interrupted) => break 'outer,
                Err(ReadlineError::Eof) => println!("End of File Error"),
                Err(_) => println!("Input Error"), 
            }
    }
    repl_setup::kill_repl();
}
