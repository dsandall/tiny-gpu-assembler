.threads 4                      ; Specify 4 threads
.data 1 2 3 4; Initial data in memory (4 bytes)

CONST R1, #4                    ; Increment for addresses (4 bytes later)
CONST R2, #4                    ; Number of iterations
CONST R3, #0                    ; Base address
CONST R4, #2                    ; Multiply factor (2 for doubling)

MUL R0, %blockIdx, %blockDim    ; Compute global thread index
ADD R0, R0, %threadIdx          ; R0 = thread ID

ADD R6, R3, R0                  ; Initial address = base + thread offset

CONST R7, #0                    ; Iteration counter

LOOP:
    ; Load byte from memory
    LDR R8, R6                  ; R8 = memory[R6] (load)

    ; Add byte to itself (R8 * 2)
    MUL R8, R8, R4              ; R8 = R8 * 2

    ; Store the result 4 addresses later
    ADD R6, R6, R1              ; R6 = R6 + 4 (next store address)
    STR R6, R8                  ; memory[R6] = R8 (store)
    NOP
    NOP

    ; Increment iteration counter
    CONST R12, #1 
    ADD R7, R7, R12              ; R7 = R7 + 1

    ; Repeat if iteration counter < 4
    CMP R7, R2
    BRn LOOP                    ; Branch back to LOOP if R7 < 4

RET                             ; End of program
