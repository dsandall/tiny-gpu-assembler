.threads 4
.data 0 0 0 0          ; Memory space to store results

CONST R0, #0           ; Test result accumulator
CONST R1, #-1          ; Negative value (N)
CONST R2, #0           ; Zero value (Z)
CONST R3, #1           ; Positive value (P)
CONST R4, #0           ; Address to store test results

; --- Test BRn ---
CMP R1, R2             ; Compare negative with zero
BRn TEST_N             ; Branch if Negative flag set
CONST R0, #0           ; If branch fails, set R0 = 0 (indicating failure)
BR DONE_N
TEST_N:
CONST R0, #1           ; R0 = 1 if BRn succeeded
DONE_N:
STR R4, R0             ; Store result at memory[R4] (test BRn)
ADD R4, R4, #1         ; Increment result address

; --- Test BRz ---
CMP R2, R2             ; Compare zero with zero
BRz TEST_Z             ; Branch if Zero flag set
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_Z
TEST_Z:
CONST R0, #1           ; R0 = 1 if BRz succeeded
DONE_Z:
STR R4, R0             ; Store result at memory[R4] (test BRz)
ADD R4, R4, #1

; --- Test BRp ---
CMP R3, R2             ; Compare positive with zero
BRp TEST_P             ; Branch if Positive flag set
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_P
TEST_P:
CONST R0, #1           ; R0 = 1 if BRp succeeded
DONE_P:
STR R4, R0             ; Store result at memory[R4] (test BRp)
ADD R4, R4, #1

; --- Test BRnz (N or Z) ---
CMP R1, R2             ; Compare negative with zero
BRnz TEST_NZ           ; Branch if Negative or Zero flag set
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_NZ
TEST_NZ:
CONST R0, #1           ; R0 = 1 if BRnz succeeded
DONE_NZ:
STR R4, R0             ; Store result at memory[R4] (test BRnz)
ADD R4, R4, #1

; --- Test BRzp (Z or P) ---
CMP R2, R2             ; Compare zero with zero
BRzp TEST_ZP           ; Branch if Zero or Positive flag set
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_ZP
TEST_ZP:
CONST R0, #1           ; R0 = 1 if BRzp succeeded
DONE_ZP:
STR R4, R0             ; Store result at memory[R4] (test BRzp)
ADD R4, R4, #1

; --- Test BRnp (N or P) ---
CMP R1, R3             ; Compare negative with positive
BRnp TEST_NP           ; Branch if Negative or Positive flag set
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_NP
TEST_NP:
CONST R0, #1           ; R0 = 1 if BRnp succeeded
DONE_NP:
STR R4, R0             ; Store result at memory[R4] (test BRnp)
ADD R4, R4, #1

; --- Test BRnzp (Always) ---
CMP R1, R3             ; Compare any values (irrelevant here)
BRnzp TEST_NZP         ; Always branch
CONST R0, #0           ; If branch fails, set R0 = 0
BR DONE_NZP
TEST_NZP:
CONST R0, #1           ; R0 = 1 if BRnzp succeeded
DONE_NZP:
STR R4, R0             ; Store result at memory[R4] (test BRnzp)

RET                    ; End of program
