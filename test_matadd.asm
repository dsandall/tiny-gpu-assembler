; 0b0101000011011110 ; MUL R0, %blockIdx, %blockDim
; 0b0011000000001111 ; ADD R0, R0, %threadIdx         ; i = blockIdx * blockDim + threadIdx
; 0b1001000100000000 ; CONST R1, #0                   ; baseA (matrix A base address)
; 0b1001001000001000 ; CONST R2, #8                   ; baseB (matrix B base address)
; 0b1001001100010000 ; CONST R3, #16                  ; baseC (matrix C base address)
; 0b0011010000010000 ; ADD R4, R1, R0                 ; addr(A[i]) = baseA + i
; 0b0111010001000000 ; LDR R4, R4                     ; load A[i] from global memory
; 0b0011010100100000 ; ADD R5, R2, R0                 ; addr(B[i]) = baseB + i
; 0b0111010101010000 ; LDR R5, R5                     ; load B[i] from global memory
; 0b0011011001000101 ; ADD R6, R4, R5                 ; C[i] = A[i] + B[i]
; 0b0011011100110000 ; ADD R7, R3, R0                 ; addr(C[i]) = baseC + i
; 0b1000000001110110 ; STR R7, R6                     ; store C[i] in global memory
; 0b1111000000000000 ; RET                            ; end of kernel

.threads 8
.data 0 1 2 3 4 5 6 7          ; matrix A (1 x 8)
.data 0 1 2 3 4 5 6 7          ; matrix B (1 x 8)

MUL R0, %blockIdx, %blockDim
ADD R0, R0, %threadIdx         ; i = blockIdx * blockDim + threadIdx

CONST R1, #0                   ; baseA (matrix A base address)
CONST R2, #8                   ; baseB (matrix B base address)
CONST R3, #16                  ; baseC (matrix C base address)

ADD R4, R1, R0                 ; addr(A[i]) = baseA + i
LDR R4, R4                     ; load A[i] from global memory

ADD R5, R2, R0                 ; addr(B[i]) = baseB + i
LDR R5, R5                     ; load B[i] from global memory

ADD R6, R4, R5                 ; C[i] = A[i] + B[i]

ADD R7, R3, R0                 ; addr(C[i]) = baseC + i
STR R7, R6                     ; store C[i] in global memory

RET                            ; end of kernel