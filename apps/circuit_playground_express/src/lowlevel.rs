/// Wrapper around "low level" routines written in C and assembler

/// Functions defined in asmhelpers.s
extern "C" {
    pub fn asm_add(a: i32, b: i32) -> i32;
    fn asm_dump_registers(register_buf: *mut u8);
}

/// Functions defined in chelpers.c
extern "C" {
    pub fn c_add(a: i32, b: i32) -> i32;
}

#[repr(C)]
pub struct RegisterBuf {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,    // a.k.a. r13
    pub lr: u32,    // a.k.a. r14
    pub pc: u32,    // a.k.a. r15
}

impl RegisterBuf {
    pub fn new() -> RegisterBuf {
        RegisterBuf {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            sp: 0,    // a.k.a. r13
            lr: 0,    // a.k.a. r14
            pc: 0,    // a.k.a. r15
        }
    }

    pub fn load(&mut self) {
        unsafe {
            asm_dump_registers((self as *mut RegisterBuf) as *mut u8);
        }
    }
}
