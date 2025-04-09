
#![allow(warnings)]

use std::{env::args, process::exit};
use std::fs::File;
use std::io::prelude::*;

fn as_bytes(array: &[u8]) -> i32 {
        ((array[0] as i32) << 24) + 
        ((array[1] as i32) << 16) + 
        ((array[2] as i32) << 8)  + 
        ((array[3] as i32) << 0) 
}
fn main() {
    let mut STACK: Vec<i32> = Vec::new();
    let mut STACK_POINTER = 4096;
    let mut PROGRAM_COUNTER = 0;
    let mut RAM: [u8; 4096] = [0; 4096];

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("marz is gonna git you ");
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("File missing");
    file.read(&mut RAM).expect("sum ting wong");

    // read in 4 bytes at a time and convert to instruction
    for x in (0..=4092).rev().step_by(4) {
        let instruction = as_bytes(&RAM[x .. x+4]);
        if (x == 0) {
            if instruction as u32 == 0xdeadbeef {
                println!("{:04X} {:x}",x, instruction);
                continue;
            } else {
                println!("the cow is missing");
                exit(1);
            }
        }
        if instruction != 0 {
            STACK.push(instruction);
        }
    }
    
    STACK_POINTER = STACK.len() - 1;
    PROGRAM_COUNTER = STACK.len() - 1;

    for x in 0 .. STACK.len() {
        println!("{:04X} {:x}",x, STACK[x]);
    }
    
}
