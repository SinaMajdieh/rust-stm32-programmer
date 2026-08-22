.syntax unified
.cpu cortex-m3
.thumb

.global Reset_Handler
.global _estack

.section .isr_vector, "a", %progbits
.type vector_table, %object

vector_table:
    .word _estack
    .word Reset_Handler

.size vector_table, . - vector_table


.section .text.Reset_Handler, "ax", %progbits
.type Reset_Handler, %function

Reset_Handler:

    bl main

hang:
    b hang

.size Reset_Handler, . - Reset_Handler
