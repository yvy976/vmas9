use std::{io::{self, stdin, Write}, process::exit};

const STACK_SIZE: usize = 1024;

fn reverse(value: i32) -> i32 {
    (((value >> 0) & 0xFF) << 24)
        + (((value >> 8) & 0xFF) << 16)
        + (((value >> 16) & 0xFF) << 8)
        + (((value >> 24) & 0xFF) << 0)
}
pub fn Exit(instruction: i32) {
    let code = reverse(instruction) & 0xF;
    // println!("exit {}", code);
    // exit(code);
}

pub fn Swap(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
    let mut from = ((reverse(instruction) >> 12) & 0xFFFF) as isize;
    let mut to = (reverse(instruction) & 0xFFFF) as isize;

    // from
    from <<= 2;
    from &= 0x0FFF;
    if from & 0x800 != 0 {
        from -= 0x1000;
    }
    to <<= 2;
    to &= 0x0FFF;
    if to & 0x800 != 0 {
        to -= 0x1000;
    }
    // println!("swap {} {}, {:x}", from, to, (reverse(instruction)));
    let from_offset = (*sp as isize + from / 4) as usize;
    let to_offset = (*sp as isize + to / 4) as usize;

    stack[from_offset] ^= stack[to_offset];
    stack[to_offset] ^= stack[from_offset];
    stack[from_offset] ^= stack[to_offset];
}

pub fn Nop() {}
pub fn Input(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let mut value = String::new();
    stdin().read_line(&mut value);
    // value = value.trim();
    let ivalue: i32 = value.trim().parse::<i32>().expect("blah");
    Push(
        reverse((0b1111 << 28) | ivalue & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Stinput(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let size = reverse(instruction) & 0x0FFFFFFF;
    let mut value = String::new();
    stdin().read_line(&mut value);

    let svalue = value
        .trim()
        .split_at(size as usize)
        .0
        .parse::<i32>()
        .unwrap();

    Push(
        reverse((0b1111 << 28) | svalue & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Debug(instruction: i32) {}

pub fn Pop(instruction: i32, sp: &mut usize, len: &mut u32) {
    let mut offset = ((reverse(instruction) & 0x0FFFFFFF) & !3) as u32;

    if offset >= *len * 4 {
        offset = (*len) * 4;
    }

    *sp += (offset / 4) as usize;
    // println!("pop {} {} {:x}", offset, *sp, instruction);
}

pub fn Add(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left + right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );

    // println!(
    //     "add {} {:x}",
    //     value,
    //     (0b1111 << 28) | value & ((1 << 28) - 1)
    // );
}
pub fn Sub(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left - right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
    // println!("sub {} left: {} right: {}", value, left, right);
}
pub fn Mul(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left * right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Div(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left / right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Rem(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left % right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn And(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left & right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Or(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left | right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Xor(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left ^ right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Lsl(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left << right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Lsr(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    let value = left >> right;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Asr() {}

pub fn Neg(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let operand = stack[*sp];
    Pop(0, sp, len);
    let value = -operand;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Not(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let operand = stack[*sp];
    Pop(0, sp, len);
    let value = !operand;
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Stprint(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
    // let mut value = ((reverse(instruction) << 4) >> 4) & !2;
    let mut value = (reverse(instruction) & 0x0FFFFFFF) & !3;

    let start = *sp + (value / 4) as usize;
    // println!{"stprint {} {:x} {}", *sp, stack[*sp], value};
    for i in (start..STACK_SIZE) {
        let v = reverse(stack[i]) & 0x0FFFFFFF;
        // *sp += 1;

        // println!(" v: {:x} {:x}", v, stack[i]);
        for x in (0..=24).step_by(8) {
            let byte = ((v >> x) & 0xFF) as u8;
            let ch = byte as char;
            print!("{}", ch);
            
            if ch == '\0' {
                io::stdout().flush().expect("Unable to flush stdout");
                return;
            }
        }
        // if (v  & 0xFF) == 0 {
        //     return
        // }
    }

    // println!("stprint {:x} {:x}", value, reverse((instruction))  );
}
pub fn Call(instruction: i32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;

    Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
}
pub fn Return(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    *sp += (offset / 4) as usize;
    let value = stack[*sp + (offset / 4) as usize];
}
pub fn Goto(instruction: i32, pc: &mut i32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    if offset & 0x8000000 != 0 {
        offset -= 0x10000000;
    }
    *pc = (offset);
    // println!("goto {}", offset)
}
pub fn Eq(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let mut offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    if offset & 0x1000000 != 0 {
        offset -= 0x10000000;
    }

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    if left == right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Ne(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);

    if left != right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Lt(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);
    if left < right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Gt(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);

    if left > right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Le(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);

    if left > right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Ge(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0, sp, len);

    if left > right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Ez(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let x = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    if x == 0 {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Nz(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let x = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    if x != 0 {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Mi(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let x = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    if x < 0 {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Pl(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let offset = (reverse(instruction) & 0x0EFFFFFF) & !3;

    let x = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    if x >= 0 {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), &mut 0);
    } else {
        Nop();
    }
}
pub fn Dup(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    let value = stack[*sp + (offset / 4) as usize];
    // println!("dup {:x} {:x}", offset, value);

    Push(value, stack, sp, len, &mut false);
}
pub fn Print(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
    let format = reverse(instruction) & 0x3;
    let offset = (reverse(instruction) & 0x0FFFFFFF) & !3;

    let value = reverse(stack[*sp + (offset / 4) as usize]) & 0x0FFFFFFF;
    println!("{}", value);
    io::stdout().flush().expect("Unable to flush stdout");
    // println!("integer print {} {:x} {:x}", value, instruction, offset);
}

pub fn Dump(stack: &mut [i32; STACK_SIZE]) {
    for x in 0..STACK_SIZE {
        println!("{:04x}: {:08x}", x, reverse(stack[x]))
    }
}

pub fn Push(
    instruction: i32,
    stack: &mut [i32; STACK_SIZE],
    sp: &mut usize,
    len: &mut u32,
    cont_string: &mut bool,
) {
    //// TODO
    /// value sign extend (?)
    ///
    ///
    let mut value = ((instruction) & !0xF0);


    *sp -= 1;
    if stack[*sp] == 0 {
        *len += 1;
    }
    stack[*sp] = (instruction);


}
