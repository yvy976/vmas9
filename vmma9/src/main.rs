#![allow(warnings)]

pub mod Instructions;
pub mod instructions;

use std::fs::File;
use std::io::prelude::*;
use std::iter::Product;
use std::{collections::HashMap, env::args, process::exit};

use Instructions::{Ez, Nz};

fn as_bytes(array: &[u8]) -> i32 {
    ((array[0] as i32) << 24)
        + ((array[1] as i32) << 16)
        + ((array[2] as i32) << 8)
        + ((array[3] as i32) << 0)
}
// fn as_bytes(array: &[u8]) -> i32 {
//         ((array[0] as i32) << 0)
//         + ((array[1] as i32) << 8)
//         + ((array[2] as i32) << 16)
//         + ((array[3] as i32) << 24)
// }
fn Miscellaneous(instruction: i32, stack: &mut [i32; 1024], sp:  &mut usize, len: &mut u32) -> bool {
    let opcode = instruction & 0xFF;
    match opcode {
        0 => Instructions::Exit(instruction),
        0x01 => Instructions::Swap(instruction, stack, sp),
        0x02 => Instructions::Nop(),
        4 => Instructions::Input(stack, sp, len),
        5 => Instructions::Stinput(instruction, stack, sp, len),
        // 15 => Instructions::Debug(instruction),
        _ => println!("sumthing wrong {} {}", instruction, opcode),
    }
    return opcode == 0 || opcode == 2;
}

fn Arithmetic(instruction: i32, stack: &mut [i32; 1024], sp:  &mut usize, len: &mut u32) {
    let opcode = (instruction & 0xF0) >> 4;
    let operator = instruction & 0x0F;


    if opcode == 2 {
        match operator {
            0 => Instructions::Add(stack, sp, len),
            1 => Instructions::Sub(stack, sp, len),
            2 => Instructions::Mul(stack, sp, len),
            3 => Instructions::Div(stack, sp, len),
            4 => Instructions::Rem(stack, sp, len),
            5 => Instructions::And(stack, sp, len),
            6 => Instructions::Or(stack, sp, len),
            7 => Instructions::Xor(stack, sp, len),
            8 => Instructions::Lsl(stack, sp, len),
            9 => Instructions::Lsr(stack, sp, len),
            // 11 => Instructions::Asr(stack, sp, len),
            _ => println!("sum thing wrong"),
        }
    } 
    else if opcode == 3 {
        match operator {
            0 => Instructions::Neg(stack, sp, len),
            1 => Instructions::Not(stack, sp, len),
            _ => println!("sum thing wrong"),
        }
    }
}

fn If(instruction: i32, stack: &mut [i32; 1024], sp:  &mut usize, len: &mut u32, pc: &mut i32) {
    let opcode = (instruction & 0xF0) >> 4;
    let operator = (instruction & 0x0F) >> 1;

    if opcode == 8 {
        match operator {
            0 => Instructions::Eq(instruction, stack, sp, len, pc),
            1 => Instructions::Ne(instruction, stack, sp, len, pc),
            2 => Instructions::Lt(instruction, stack, sp, len, pc),
            3 => Instructions::Gt(instruction, stack, sp, len, pc),
            4 => Instructions::Le(instruction, stack, sp, len, pc),
            5 => Instructions::Ge(instruction, stack, sp, len, pc),
            _ => println!("some thign wrong"),
        }
    } else if opcode == 9 {
        match operator {
            0 => Instructions::Ez(instruction, stack, sp, len, pc),
            1 => Instructions::Nz(instruction, stack, sp, len, pc),
            2 => Instructions::Mi(instruction, stack, sp, len, pc),
            3 => Instructions::Pl(instruction, stack, sp, len, pc),
            _ => println!("some thign wrong"),
        }
    }
}

fn main() {
    let mut STACK: [i32; 1024] = [0; 1024];
    let mut STACK_POINTER: usize = 1024;
    let mut PROGRAM_COUNTER = 0;
    let mut RAM: [u8; 4096] = [0; 4096];
    let mut LENGTH = 0;

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("marz is gonna git you ");
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("File missing");
    file.read(&mut RAM).expect("sum ting wong");

    let mut cont_string = false;
    // read in 4 bytes at a time and convert to instruction
    // for mut x in (0..=4092).step_by(4) {
    let mut x: i32 = 0;
    while x <= 4093 {
        PROGRAM_COUNTER = 0;

        // if PROGRAM_COUNTER != 0 {
        //     x = PROGRAM_COUNTER; // Set x directly to the jump target
        //     PROGRAM_COUNTER = 0; // Reset for next instruction
        // } else {
        //     x += 4; // Otherwise just move to next instruction
        // }

        let instruction = as_bytes(&RAM[(x as usize)..(x + 4) as usize]); 
        // println!("pc {} {} {:x}",x, PROGRAM_COUNTER, instruction);
   
 

        if (instruction as u32  == 0x02) {
            println!("nop {:x}", instruction);
            break;
        } 
        if (x == 0) {
            // if instruction as u32 == 0xefbeadde {
            if instruction as u32 == 0xdeadbeef {
                x += 4;
                continue;
            } else {
                println!("the cow is missing");
                exit(1);
            }
        }
        // let opcode = instruction as u32 >> 28;
        let opcode = (instruction & 0xF0) >> 4;
        // if (instruction != 0) {
        //     println!("{} {:x} {}", x, instruction, opcode);

        // }



            match opcode {
                0 => {
                    if Miscellaneous(instruction, &mut STACK, &mut STACK_POINTER, &mut LENGTH) {
                        // println!("##");
                        break;
                    }
                }
                1 =>     Instructions::Pop(instruction, &mut STACK_POINTER, &mut LENGTH),
                2 | 3 => Arithmetic(instruction, &mut STACK, &mut STACK_POINTER, &mut LENGTH),
                4 =>     Instructions::Stprint(instruction, &mut STACK, &mut STACK_POINTER),
                5 =>     Instructions::Call(instruction, &mut PROGRAM_COUNTER, &mut STACK, &mut STACK_POINTER, &mut LENGTH, &mut x),
                6 =>     Instructions::Return(instruction, &mut STACK, &mut STACK_POINTER, &mut x, &mut LENGTH),
                7 =>     Instructions::Goto(instruction, &mut PROGRAM_COUNTER),
                8 | 9 => If(instruction, &mut STACK, &mut STACK_POINTER, &mut LENGTH, &mut PROGRAM_COUNTER),
                12 =>    Instructions::Dup(instruction, &mut STACK, &mut STACK_POINTER, &mut LENGTH),
                13 =>    Instructions::Print(instruction, &mut STACK, &mut STACK_POINTER),
                14 =>    Instructions::Dump(&mut STACK),
                15 =>    Instructions::Push(instruction, &mut STACK, &mut STACK_POINTER, &mut LENGTH, &mut cont_string),

                _ => print!(""),
            }
            if PROGRAM_COUNTER != 0 {
                x += PROGRAM_COUNTER;
            } else {
                x += 4;
            }
    }
    // println!("{:x?}", STACK);   
    // println!("{}, {:X}", STACK_POINTER, STACK[1016]);


    // println!("ghuh");
}
