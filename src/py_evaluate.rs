use colored::Colorize;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::string::String;

use duct::cmd;

fn run_pycode(entire_string: &String) -> String {
    create_and_write(entire_string);

    let py_output = cmd!("python3", "repl.py").read();
    py_output.expect("Error evaluating python code")
}

fn pre_run_input(entire_string: &String) -> Result<String, Error> {
    create_and_write(entire_string);
    cmd!("python3", "repl.py").read()
}

fn create_and_write(string_to_write: &String) {
    let mut executable_file = File::create("repl.py").expect("Could not create new file");
    executable_file
        .write_all(string_to_write.as_bytes())
        .unwrap();
}

pub fn evaluate_code(entire_string: &mut String, input_string: String) -> String {
    let mut copy_of_entire = entire_string.clone();
    copy_of_entire.push_str(&input_string);

    let input_check = pre_run_input(&copy_of_entire);

    match input_check {
        Ok(input_output) => {
            if input_output.trim().is_empty() {
                entire_string.push_str(&input_string);
            }

            println!("{}", run_pycode(&copy_of_entire).purple().italic());
            entire_string.to_string()
        }
        Err(_) => {
            let output = String::from("");
            entire_string.push_str(&output);
            entire_string.to_string()
        }
    }
}
