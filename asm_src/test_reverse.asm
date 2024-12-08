.threads 4                      ; Specify 4 threads
.data 1 2 3 4                   ; Initial data in memory (4 bytes)

CONST R1, #4                    ; Increment for addresses (4 bytes)
CONST R2, #4                    ; Number of elements (4 items in the array)
CONST R3, #0                    ; Base address (starting address)
CONST R4, #3                    ; Offset for reverse indexing (last element)

MUL R0, %blockIdx, %blockDim    ; Compute global thread index
ADD R0, R0, %threadIdx          ; R0 = thread ID

CONST R5, #0                    ; Initial index for reverse (starting from 3)
ADD R6, R3, R5                  ; R6 = base + reverse index (first element is at index 3)

ADD R7, R3, R0                  ; Initial address for storing reversed data

LOOP:
    ; Load value from memory (reversed index)
    LDR R8, R6                  ; R8 = memory[R6] (load)

    ; Store the value at the new location
    STR R8, R7                  ; memory[R7] = R8 (store reversed value)

    ; Update the reversed index
    SUB R5, R5, R1              ; Decrement reverse index by 1 (moving backwards)

    ; Update the store address
    ADD R7, R7, R1              ; Move store address forward by 4 bytes (next location)

    ; Update the load address
    ADD R6, R6, R1              ; Move load address forward by 4 bytes (next location)

    ; Check if all threads have processed
    CMP R0, R2
    BRn LOOP                    ; Continue looping until all threads complete

RET                             ; End of program
