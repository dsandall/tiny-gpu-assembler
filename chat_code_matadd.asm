.threads 4
.data 1 2 3 4            ; vector A (4 elements)
.data 5 6 7 8            ; vector B (4 elements)
.data 0 0 0 0            ; vector C (result, 4 elements)

MUL R0, %blockIdx, %blockDim  ; R0 = blockIdx * blockDim (not needed here, but could be used for larger grids)
ADD R0, R0, %threadIdx       ; R0 = blockIdx * blockDim + threadIdx, this gives the thread index (i)

CONST R1, #0                 ; baseA (base address of vector A)
CONST R2, #4                 ; baseB (base address of vector B)
CONST R3, #8                 ; baseC (base address of vector C)

ADD R4, R1, R0               ; addr(A[i]) = baseA + i, load address of A[i]
LDR R4, R4                    ; load A[i] from global memory into R4

ADD R5, R2, R0               ; addr(B[i]) = baseB + i, load address of B[i]
LDR R5, R5                    ; load B[i] from global memory into R5

ADD R6, R4, R5               ; C[i] = A[i] + B[i], perform the addition

ADD R7, R3, R0               ; addr(C[i]) = baseC + i, store the result at C[i]
STR R7, R6                    ; store C[i] in global memory

RET                           ; end of kernel
