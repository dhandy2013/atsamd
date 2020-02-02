// Define assembly-language subroutines callable from Rust
.text

// If you don't include the .global directive then the linker will fail.
.global asm_add
// If you don't include the .type directive then the compiler will generate a
// "blx" instruction instead of a "bl" instruction at the place where asm_add
// is called, and that will result in a CPU fault.
.type asm_add, %function
asm_add:                        // Name of subroutine
    add     r0, r0, r1          // Add parameter 1 and 2
    bx      lr                  // Return value in r0

.global asm_dump_registers
.type asm_dump_registers, %function
asm_dump_registers:
    // r0: Address of an a array of 16 32-bit words to store the result
    str     r0, [r0, #0]
    str     r1, [r0, #4]
    str     r2, [r0, #8]
    str     r3, [r0, #12]
    str     r4, [r0, #16]
    str     r5, [r0, #20]
    str     r6, [r0, #24]
    str     r7, [r0, #28]
    // Can't store r8 through r15 directly to memory
    push    {r1}
    mov     r1, r8
    str     r1, [r0, #32]
    mov     r1, r9
    str     r1, [r0, #36]
    mov     r1, r10
    str     r1, [r0, #40]
    mov     r1, r11
    str     r1, [r0, #44]
    mov     r1, r12
    str     r1, [r0, #48]
    mov     r1, sp
    str     r1, [r0, #52]
    mov     r1, lr
    str     r1, [r0, #56]
    mov     r1, pc
    str     r1, [r0, #60]
    pop     {r1}
    bx      lr

.end
