use std::{io::{self, stdin, Write}, process::exit};

const STACK_SIZE: usize = 1024;

fn reverse(value: i32) -> i32 {
    (((value >> 0) & 0xFF) << 24)
        + (((value >> 8) & 0xFF) << 16)
        + (((value >> 16) & 0xFF) << 8)
        + (((value >> 24) & 0xFF) << 0)
}
pub fn Exit(instruction: i32) {
    let code = reverse(instruction) & 0xFF;
    println!("exit {}", code);
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
    // println!("value is {}", ivalue);
    Push(
        reverse((0b1111 << 28) | ivalue & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
    // println!("input {}", *sp);
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

    if offset & 0x08000000 != 0 {
        offset -= 0x10000000;
    }
    if offset > *len * 4 {
        // offset = (*len) * 4;
        *sp = STACK_SIZE;
    } else {
        // println!("also here {} {}", offset, len);
    *sp += (offset / 4) as usize;


    }
    // *len -= (offset/4);
    // if *sp >= STACK_SIZE {
    //     *sp = STACK_SIZE-1;
    // }
    // println!("pop {} {} {:x}", offset, *sp, instruction);
}

pub fn Add(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    println!("{:x?}", stack);
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    // if (*sp == STACK_SIZE) {
        Pop(0x4000010, sp, len);

    // } 
    // else {
    //     // Pop(0x0000010, sp, len);
         
    // }
    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;

    println!("left right {} {}", left, right);


    // Pop(0x4000010, sp, len);

    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
    let mut value = left + right;


    //     if value & 0x8000000 != 0 {
    //     value -= 0x10000000;
    // }

    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );




}
pub fn Sub(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
    let value = left * right;
    println!("mult {} {}", left, right);
    Push(
        reverse((0b1111 << 28) | value & ((1 << 28) - 1)),
        stack,
        sp,
        len,
        &mut false,
    );
}
pub fn Div(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut right = reverse(stack[*sp]) & 0x0FFFFFFF;
    Pop(0x4000010, sp, len);

    let mut left = reverse(stack[*sp]) & 0x0FFFFFFF;
    if left & 0x8000000 != 0 {
        left -= 0x10000000;
    }
    if right & 0x8000000 != 0 {
        right -= 0x10000000;
    }
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
    Pop(0x4000010, sp, len);

    let mut operand = reverse(stack[*sp])&0x0FFFFFFF;

    if operand & 0x8000000 != 0 {
        operand -= 0x10000000;
    }

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
    Pop(0x4000010, sp, len);

    let mut operand = reverse(stack[*sp])&0x0FFFFFFF;

    let mut value = !operand;
    if value & 0x8000000 != 0 {
        value -= 0x10000000;
    }


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
    for i in (start..STACK_SIZE) {
        let v = reverse(stack[i]) & 0x0FFFFFFF;
        // *sp += 1;

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

}
pub fn Call(instruction: i32, pc: &mut i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32, next: &mut i32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    // println!("call {} {:x}", offset, next);
    Push(
        reverse((0b1111 << 28) | (*next) & ((1 << 28) - 1) & !3),
        stack,
        sp,
        len,
        &mut false,
    );


    Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
}
pub fn Return(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, pc: &mut i32, len: &mut u32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;

    let wp = stack[(*sp + (offset/4) as usize)];
    let value = (reverse(wp) & 0x0FFFFFFF) & !3;

    
    Pop(reverse((0b0001 << 28) | (offset) & ((1 << 28)-1)& !3), sp, len);

    *pc = value ;

}
pub fn Goto(instruction: i32, pc: &mut i32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    // println!("{} {:x}", offset, reverse(instruction));

    if offset & 0x8000000 != 0 {
        offset -= 0x10000000;
    }

    *pc = (offset);
}
pub fn Eq(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let mut offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    if (offset & 0x1000000) != 0 {
        
        offset |= 0xFE000000u32 as i32;  
    }

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp+1]) & 0x0FFFFFFF;
    // Pop(0, sp, len);
    if left == right {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {
        Nop();
    }
}
pub fn Ne(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;

    let left = reverse(stack[*sp+1]) & 0x0FFFFFFF;

    if left != right {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {
        Nop();
    }
}
pub fn Lt(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;

    let left = reverse(stack[*sp+1]) & 0x0FFFFFFF;

    if left < right {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {
        Nop();
    }
}
pub fn Gt(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp+1]) & 0x0FFFFFFF;
    // Pop(0, sp, len);

    if left > right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), pc);
    } else {
        Nop();
    }
}
pub fn Le(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let right = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);
    let left = reverse(stack[*sp+4]) & 0x0FFFFFFF;
    // Pop(0, sp, len);

    if left <= right {
        Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), pc);
    } else {
        Nop();
    }
}
pub fn Ge(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {

    let mut offset = (reverse(instruction) & 0x01FFFFFF) & !3;
    let right = reverse(stack[*sp]) & 0x0FFFFFFF;

    let left = reverse(stack[*sp+1]) & 0x0FFFFFFF;
    // if offset & 0x0FFFFFFF != 0 {
    //     offset -= 0x10000000;
    // }

    if (offset & 0x1000000) != 0 {
        
        offset |= 0xFE000000u32 as i32;  
    }

    println!("ge {} {}", left, right);


    if left >= right {

        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {

        Nop();
    }
}
pub fn Ez(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let mut x = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);
    if x & 0x08000000 != 0 {
        x -= 0x10000000;
    }
    if x == 0 {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);

    } else {
        Nop();
    }
}
pub fn Nz(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let mut x = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);
    if x & 0x08000000 != 0 {
        x -= 0x10000000;
    }
    if x != 0 {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);

    } else {
        Nop();
    }
}
pub fn Mi(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32, pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let x = reverse(stack[*sp]) & 0x0FFFFFFF;
    // Pop(0x4000010, sp, len);

    if x < 0 {
        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);

    } else {
        Nop();
    }
}
pub fn Pl(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32, pc: &mut i32) {
    let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

    let mut x = reverse(stack[*sp]) & 0x0FFFFFFF;

    if x & 0x08000000 != 0 {
        x -= 0x10000000;
    }
    // Pop(0x4000010, sp, len);

    if x >= 0 {

        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {
        Nop();
    }

    //     println!("pl {}", offset);
    // println!("{:x?}", stack);
    // println!("{} {:x}", sp, stack[*sp]);
}
pub fn Dup(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;
    let value = stack[*sp + (offset / 4) as usize];
    // println!("dup {:x} {:x}", offset, value);

    Push(value, stack, sp, len, &mut false);
}
pub fn Print(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
    let format = reverse(instruction) & 0x3;
    let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;

    if offset & 0x08000000 != 0 {
        offset -= 0x10000000;
    }

    // offset -= 1;
    // let value = reverse(stack[*sp + (offset / 4) as usize]) & 0x0FFFFFFF;
    let mut value = reverse(stack[(*sp as i32 + (offset/4)) as usize]) & 0x0FFFFFFF;
    if value & 0x08000000 != 0 {
        value -= 0x10000000;
    }
    // println!("vvv {}", value);
    println!("{}", value );

    io::stdout().flush().expect("Unable to flush stdout");
    println!("{:x?}", stack);
    println!("sp+offset = {} ", *sp as i32 + (offset/4));

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
