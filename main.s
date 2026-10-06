.section .text.init, "ax"
.global _start

.equ COP_STATUS, 0x60007004
.equ COP_CTRL, 0x60007004
.equ SLEEP, 0x80000000
.equ COPSLEEP, 0x80000000
.equ PROC_ID, 0x60000000

_start:
    @ disable interrupts and enter system mode
    msr cpsr_c, #0xdf

    @ which processor is this?
    ldr r0, =PROC_ID
    ldrb r0, [r0]
    cmp r0, #0x55
    beq cpu

cop:
    @ put cop to sleep
    ldr r2, =COP_CTRL
    mov r1, #SLEEP
    str r1, [r2]
    @ wait for cop to go to sleep
    nop
    nop
    nop

    @ if woken up, go back to sleep
    b cop

cpu:
    @ check if cop is asleep
    ldr r4, =COP_STATUS
3:
    ldr r3, [r4]
    tst r3, #COPSLEEP
    beq 3b

    @ zero out bss
    ldr r2, =_bssend
    ldr r1, =_bssbegin
    mov r0, #0
1:
    cmp r2, r1
    strhi r0, [r1], #4
    bhi 1b

    @ fill stack
    ldr r1, =_stackend
    ldr r2, =_stackbegin
    ldr r0, =0xDEADBEEF
1:
    cmp r1, r2
    strhi r0, [r2], #4
    bhi 1b

    ldr sp, =_stackend

    bl rust_main

hang:
    b hang
