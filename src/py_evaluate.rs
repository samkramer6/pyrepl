use std::fs::File;
use std::io::Write;
use std::string::String;

use duct::cmd;

fn run_pycode(entire_string: &String) -> String {

    create_and_write(&entire_string);

    let py_output  = cmd!("python","repl.py").read();

    match py_output {
        Ok(output) => output,
        Err(_error_message) => String::new(),
    } 

}

fn pre_run_input(entire_string: &String) -> u32 {

    create_and_write(&entire_string);

    let py_output  = cmd!("python","repl.py").read();

    match py_output {
        Ok(output) => output.trim().len().try_into().expect("No Input"),
        Err(_error_message) => String::new().trim().len().try_into().unwrap(),
    }

}

fn create_and_write(string_to_write: &String) {

    let mut executable_file = File::create("repl.py").expect("Could not create new file");
    executable_file.write_all(string_to_write.as_bytes()).unwrap();

}

pub fn evaluate_code(entire_string: &mut String, input_string: &String) -> String {
    
    let mut copy_of_entire = entire_string.clone();
    copy_of_entire.push_str(&input_string); 
            
    let input_output_length = pre_run_input(&copy_of_entire);
            
    if input_output_length == 0 {
        entire_string.push_str(&input_string);
    } 
            
    let output = run_pycode(&copy_of_entire);
    println!("{}", output);
    
    entire_string.to_string()
}
