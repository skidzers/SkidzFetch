// Load the ability to format colors in term
use colored::Colorize;
// Basic
use crossterm::{ cursor, execute, terminal };
use std::io::stdout;
use sysinfo::{ System };
use chrono::Local;

fn main(){
    // refesh lol
    let mut sys=System::new_all();
    sys.refresh_all();

    // Find a fine art
    let ascii = include_str!(".././ascii/tux.txt");
    
    // Hardcoded fetch information
    //-- User
        let uname = whoami::username().unwrap();
    //--Sys
        let time = Local::now().format("%I:%M%P").to_string();
        let os = System::name().unwrap();
    //--DE/WM
        let raw_de = whoami::desktop_env().expect("BRR").to_string();
        let de = raw_de.strip_prefix("Unknown: ").unwrap_or(&raw_de);

    // clear screen
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    // Output the fetch
    execute!(stdout(), cursor::MoveTo(0, 2)).unwrap();
    println!("{:-^17}", time.cyan());
    println!("{:-<10}{}\n---\n{}\n{}\n---", ascii, uname, os, de);
    color();

}

fn color(){
    println!("{} {} {}", "(*v)<".cyan(), "(*v)<".blue(), "(*v)<".white());
}
