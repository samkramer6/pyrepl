extern crate cfonts;
use cfonts::{say, Options, Colors, Fonts};
use colored::Colorize;
use std::{ thread, time };
use std::path::Path;
use std::fs;

pub fn welcome_message() {

    // Setup Powershell
        clearscreen::clear().expect("could not clear screen");
        say(Options {
            text: String::from("PyREPL"),
            font: Fonts::FontChrome,
            colors: vec![Colors::Candy],
            ..Options::default()
        });        
        
        println!("{}", "Welcome to PyREPL".purple().italic());

        println!("\n======================== \n")

}

pub fn kill_repl() {

    // Clearscreen
        clearscreen::clear().unwrap();
        println!("{} ", "Closing PyREPL. Goodbye!".purple().italic()); 

        thread::sleep(time::Duration::from_millis(1500)); 

        clearscreen::clear().unwrap();

    // Delete executable file
        if Path::new("repl.py").exists() {
                fs::remove_file("repl.py").expect("Could not remove executable file");
        } 
}
