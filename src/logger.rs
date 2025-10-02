use std::io::Write;
use std::{fs::{OpenOptions}};
use chrono::{Local, Timelike};

fn get_output() -> std::fs::File {
    OpenOptions::new().write(true).open("CONOUT$").expect("Cannot open CONOUT$")
}

pub fn info(msg: &str) {
    writeln!(&get_output(), "[{}:{} INFO] : {}", Local::now().hour12().1, Local::now().minute(), msg).expect("Cannot write");
}

pub fn warn(msg: &str) {
    writeln!(&get_output(), "[{}:{} WARN] : {}", Local::now().hour12().1, Local::now().minute(), msg).expect("Cannot write");
}

pub fn error(msg: &str) {
    writeln!(&get_output(), "[{}:{} ERROR] : {}", Local::now().hour12().1, Local::now().minute(), msg).expect("Cannot write");
}