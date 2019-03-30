use std::io::{self, Read};
use self::command::Command;
use crate::bus::Bus;
use crate::processor::Processor;

pub mod command;

pub fn interpret<H: Bus>(cpu: &Processor, bus: &H) {
    loop {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input);
        if let Some(cmd) = Command::parse(input.as_str()) {
            execute(cpu, bus, cmd);
        } else {
            println!("ayo read the fucking help");
        }
    }
}

pub fn execute<H: Bus>(cpu: &Processor, bus: &H, cmd: Command) {
}