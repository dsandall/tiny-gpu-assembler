.threads 32
; matrix A (1 x 32)
.data 1 2 3 4 5 6 7 8
.data 11 12 13 14 15 16 17 18
.data 21 22 23 24 25 26 27 28
.data 31 32 33 34 35 36 37 38
; matrix B (1 x 32)
.data 41 42 43 44 45 46 47 48
.data 51 52 53 54 55 56 57 58
.data 61 62 63 64 65 66 67 68
.data 71 72 73 74 75 76 77 78
; matrix C (1 x 32):
                               ;@(reset):
                               ;registers[13] <= block_id;          // %blockIdx
                               ;registers[14] <= THREADS_PER_BLOCK; // %blockDim
                               ;registers[15] <= THREAD_ID;         // %threadIdx
MUL R0, %blockIdx, %blockDim
ADD R0, R0, %threadIdx         ; i = blockIdx * blockDim + threadIdx

CONST R1, #0                   ; baseA (matrix A base address)
CONST R2, #32                  ; baseB (matrix B base address)
CONST R3, #64                  ; baseC (matrix C base address)

ADD R4, R1, R0                 ; addr(A[i]) = baseA + i
LDR R4, R4                     ; load A[i] from global memory

ADD R5, R2, R0                 ; addr(B[i]) = baseB + i
LDR R5, R5                     ; load B[i] from global memory

ADD R6, R4, R5                 ; C[i] = A[i] + B[i]

ADD R7, R3, R0                 ; addr(C[i]) = baseC + i
STR R7, R6                     ; store C[i] in global memory

RET                            ; end of kernel
