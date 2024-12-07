.threads 4
.data 0 0 0 13 2 0 0 13 0 0 0 13 16 0 0 13  ; Initial array

MUL R0, %blockIdx, %blockDim           ; i = blockIdx * blockDim
ADD R0, R0, %threadIdx                 ; i = blockIdx * blockDim + threadIdx
CONST R1, #4                           ;
MUL R0, R1, R0                         ; i * 4 (start each thread at 0,4,8,12,16...)

LDR R5, R0                             ; load data[i] from memory

CMP R1, R1 ; compare equal
BR WRITE ; Comparisons tested for equal behavior
RET ; terminate

; stuff that happens if no branch
WRITE:
CONST R12 #1
ADD R5, R5, R12

CONST R12 #3
ADD R1, R0, R12 ; generate store address
STR R1, R5 ; store updated data

RET




