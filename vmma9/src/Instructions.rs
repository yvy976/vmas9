use std::{io::{self, stdin, Write}, process::exit};

const STACK_SIZE: usize = 1024;

pub fn reverse(value: i32) -> i32 {
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

// pub fn Add(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
//     // 1) Read & sign‑extend the right operand
//     let raw_right = stack[*sp];
//     let mut right = reverse(raw_right) & 0x0FFF_FFFF;
//     if right & 0x0800_0000 != 0 {
//         right -= 0x1000_0000;
//     }
//     // 2) Pop it (one word = 4 bytes)
//     *sp += 1;
//     *len = len.saturating_sub(1);

//     // 3) Read & sign‑extend the left operand
//     let raw_left = stack[*sp];
//     let mut left = reverse(raw_left) & 0x0FFF_FFFF;
//     if left & 0x0800_0000 != 0 {
//         left -= 0x1000_0000;
//     }
//     // 4) Pop it
//     *sp += 1;
//     *len = len.saturating_sub(1);

//     // 5) Compute and push the result
//     let sum = left + right;
//     Push(
//         reverse((0b1111 << 28) | (sum & ((1 << 28) - 1))),
//         stack,
//         sp,
//         len,
//         &mut false,
//     );
// }
/// Pop the top two 28‑bit values, add them, and push the 28‑bit result.
pub fn Add(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
    // ---- right operand ----
    // 1) read raw 32‑bit word
    let raw_r = stack[*sp];
    // 2) extract bottom 28 bits
    let mut r = reverse(raw_r) & 0x0FFF_FFFF;
    // 3) sign‑extend 28→32
    if r & 0x0800_0000 != 0 {
        r = r.wrapping_sub(0x1000_0000);
    }
    // 4) pop 1 word
    *sp = (*sp).saturating_add(1).min(STACK_SIZE - 1);
    *len = len.saturating_sub(1);

    // ---- left operand ----
    let raw_l = stack[*sp];
    let mut l = reverse(raw_l) & 0x0FFF_FFFF;
    if l & 0x0800_0000 != 0 {
        l = l.wrapping_sub(0x1000_0000);
    }
    *sp = (*sp).saturating_add(1).min(STACK_SIZE - 1);
    *len = len.saturating_sub(1);

    // ---- add & push result ----
    let sum = l.wrapping_add(r);
    Push(
        reverse((0b1111 << 28) | (sum & 0x0FFF_FFFF)),
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
    // ---- right operand ----
    let raw_r = stack[*sp];
    let mut r = reverse(raw_r) & 0x0FFF_FFFF;
    if r & 0x0800_0000 != 0 {
        r = r.wrapping_sub(0x1000_0000);
    }
    *sp = (*sp).saturating_add(1).min(STACK_SIZE - 1);
    *len = len.saturating_sub(1);

    // ---- left operand ----
    let raw_l = stack[*sp];
    let mut l = reverse(raw_l) & 0x0FFF_FFFF;
    if l & 0x0800_0000 != 0 {
        l = l.wrapping_sub(0x1000_0000);
    }
    *sp = (*sp).saturating_add(1).min(STACK_SIZE - 1);
    *len = len.saturating_sub(1);

    // ---- divide & push ----
    let q = l / r;
    Push(
        reverse((0b1111 << 28) | (q & 0x0FFF_FFFF)),
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
    // 1) Read & sign‑extend top
    let raw = stack[*sp];
    let mut x = reverse(raw) & 0x0FFF_FFFF;
    if x & 0x0800_0000 != 0 {
        x -= 0x1000_0000;
    }
    // 2) Pop it
    *sp += 1;
    *len = len.saturating_sub(1);

    // 3) Compute and 4) Push
    let neg = -x;
    Push(
        reverse((0b1111 << 28) | (neg & ((1 << 28) - 1))),
        stack,
        sp,
        len,
        &mut false,
    );
}
// pub fn Neg(stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32) {
//     // 1) Read & sign‑extend the top value
//     let raw = stack[*sp];
//     let mut x = reverse(raw) & 0x0FFF_FFFF;
//     if x & 0x0800_0000 != 0 {
//         x -= 0x1000_0000;
//     }

//     // 2) Pop that one word (4 bytes)
//     *sp += 1;
//     *len = len.saturating_sub(1);

//     // 3) Compute negation
//     let neg = -x;

//     // 4) Push it back
//     Push(
//         reverse((0b1111 << 28) | (neg & ((1 << 28) - 1))),
//         stack,
//         sp,
//         len,
//         &mut false,
//     );
// }

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
pub fn Return(
    instruction: i32,
    stack: &mut [i32; STACK_SIZE],
    sp: &mut usize,
    pc: &mut i32,
    len: &mut u32,
) {
    // 1) decode how many BYTES of locals to pop (rounded down to 4)
    let offset = (reverse(instruction) & 0x0FFF_FFFF) & !3;

    // 2) read the saved return‐address word from the top of the locals frame
    let saved = stack[*sp + (offset / 4) as usize];
    let ret_pc = (reverse(saved) & 0x0FFF_FFFF) & !3;

    // 3) pop the locals frame
    Pop(
        reverse((0b0001 << 28) | offset),
        sp,
        len,
    );

    // 4) pop the 4‑byte return‐address itself
    Pop(
        reverse((0b0001 << 28) | 4),
        sp,
        len,
    );

    // 5) restore the program counter
    *pc = ret_pc;
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
pub fn Le(
    instruction: i32,
    stack: &mut [i32; STACK_SIZE],
    sp: &mut usize,
    len: &mut u32,
    pc: &mut i32,
) {
    // decode signed 25‑bit offset (low 25 bits of the instruction, aligned to 4)
    let mut offset = (reverse(instruction) & 0x01FF_FFFF) & !3;
    // sign‑extend if bit 24 is set
    if offset & 0x0100_0000 != 0 {
        offset = offset.wrapping_sub(0x0200_0000);
    }

    let right = reverse(stack[*sp]) & 0x0FFF_FFFF;
    let left  = reverse(stack[*sp + 1]) & 0x0FFF_FFFF;

    if left <= right {
        // re‑encode as a Goto instruction and jump
        *pc = reverse((0x0111 << 28) | (offset & 0x0FFF_FFFF));
    } else {
        Nop();
    }
}

// pub fn Le(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
//     let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

//     let right = reverse(stack[*sp]) & 0x0FFFFFFF;
//     // Pop(0x4000010, sp, len);
//     let left = reverse(stack[*sp+4]) & 0x0FFFFFFF;
//     // Pop(0, sp, len);

//     if left <= right {
//         Goto((0x0111 << 28) | (offset & ((1 << 28) - 1)), pc);
//     } else {
//         Nop();
//     }
// }
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

    //println!("ge {} {}", left, right);


    if left >= right {

        Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);
    } else {

        Nop();
    }
}
/// Branch if top‑of‑stack is zero (empty==zero).
pub fn Ez(
    instruction: i32,
    stack: &mut [i32; STACK_SIZE],
    sp: &mut usize,
    _len: &mut u32,
    pc: &mut i32,
) {
    // If stack is empty, treat as zero; otherwise sign‑extend the top word
    let is_zero = if *sp >= STACK_SIZE {
        true
    } else {
        let mut x = reverse(stack[*sp]) & 0x0FFF_FFFF;
        if x & 0x0800_0000 != 0 {
            x = x.wrapping_sub(0x1000_0000);
        }
        x == 0
    };

    if is_zero {
        // Reuse your Goto decoder
        Goto(instruction, pc);
    }
}

/// Branch if top‑of‑stack is non‑zero (empty==zero so empty does not jump).
pub fn Nz(
    instruction: i32,
    stack: &mut [i32; STACK_SIZE],
    sp: &mut usize,
    _len: &mut u32,
    pc: &mut i32,
) {
    // Only read the top when there *is* something on the stack
    if *sp < STACK_SIZE {
        let mut x = reverse(stack[*sp]) & 0x0FFF_FFFF;
        if x & 0x0800_0000 != 0 {
            x = x.wrapping_sub(0x1000_0000);
        }
        if x != 0 {
            Goto(instruction, pc);
        }
    }
}

// pub fn Ez(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
//     let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

//     let mut x = reverse(stack[*sp]) & 0x0FFFFFFF;
//     // Pop(0x4000010, sp, len);
//     if x & 0x08000000 != 0 {
//         x -= 0x10000000;
//     }
//     if x == 0 {
//         Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);

//     } else {
//         Nop();
//     }
// }
// pub fn Nz(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize, len: &mut u32,pc: &mut i32) {
//     let offset = (reverse(instruction) & 0x01FFFFFF) & !3;

//     let mut x = reverse(stack[*sp]) & 0x0FFFFFFF;
//     // Pop(0x4000010, sp, len);
//     if x & 0x08000000 != 0 {
//         x -= 0x10000000;
//     }
//     if x != 0 {
//         Goto(reverse((0x0111 << 28) | (offset & ((1 << 28) - 1))), pc);

//     } else {
//         Nop();
//     }
// }
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
    // decode signed 28‑bit byte offset from the instruction
    let mut raw = reverse(instruction) & 0x0FFF_FFFF;
    if raw & 0x0800_0000 != 0 {
        raw = raw.wrapping_sub(0x1000_0000);
    }
    // convert to a word offset
    let word_off = (raw / 4) as isize;

    // compute the absolute stack index
    let idx = (*sp as isize).saturating_add(word_off);
    // safe‐read: if it's outside [0 .. STACK_SIZE), treat as zero
    let mut val = 0;
    if (0..(STACK_SIZE as isize)).contains(&idx) {
        let mut v = reverse(stack[idx as usize]) & 0x0FFF_FFFF;
        // sign‑extend that 28‑bit value to 32 bits
        if v & 0x0800_0000 != 0 {
            v = v.wrapping_sub(0x1000_0000);
        }
        val = v;
    }

    println!("{}", val);
    io::stdout().flush().unwrap();
}

// pub fn Print(instruction: i32, stack: &mut [i32; STACK_SIZE], sp: &mut usize) {
//     let format = reverse(instruction) & 0x3;
//     let mut offset = (reverse(instruction) & 0x0FFFFFFF) & !3;

//     if offset & 0x08000000 != 0 {
//         offset -= 0x10000000;
//     }

//     // offset -= 1;
//     // let value = reverse(stack[*sp + (offset / 4) as usize]) & 0x0FFFFFFF;
//     let mut value = reverse(stack[(*sp as i32 + (offset/4)) as usize]) & 0x0FFFFFFF;
//     if value & 0x08000000 != 0 {
//         value -= 0x10000000;
//     }
//     // println!("vvv {}", value);
//     println!("{}", value );

//     io::stdout().flush().expect("Unable to flush stdout");
//     println!("{:x?}", stack);
//     println!("sp+offset = {} ", *sp as i32 + (offset/4));

//     // println!("integer print {} {:x} {:x}", value, instruction, offset);
// }

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