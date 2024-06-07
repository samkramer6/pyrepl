use std::fs::File;
use std::io::Write;
use duct::cmd;

fn run_pycode(entire_string: &String) -> String {

    create_and_write(&entire_string);

    let py_output = cmd!("python","repl.py")
        .stderr_to_stdout()
        .read()
        .expect("Could Not Run REPL Code in Python");    
    
    py_output.to_string()
}

fn create_and_write(string_to_write: &String) {

    let mut executable_file = File::create("repl.py").expect("Could not create new file");
    executable_file.write_all(string_to_write.as_bytes()).unwrap();

}

pub fn evaluate_code(entire_string: &mut String, input_string: &String) -> String {
    
    let mut copy_of_entire = entire_string.clone();
    copy_of_entire.push_str(&input_string); 
            
    let output1 = run_pycode(&copy_of_entire);
            
            
    if output1.trim().len() == 0 {
        entire_string.push_str(&input_string);
    } 
            
    let output = run_pycode(&copy_of_entire);
    println!("{}", output);
    
    entire_string.to_string()
}
