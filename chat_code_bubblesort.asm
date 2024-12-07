.threads 4
.data 10 20 30 40              ; array of numbers (4 elements)

MUL R0, %blockIdx, %blockDim
ADD R0, R0, %threadIdx         ; i = blockIdx * blockDim + threadIdx

CONST R1, #2                   ; constant value to subtract (2)
CONST R2, #0                   ; base address of the array (start of .data)

ADD R3, R2, R0                 ; addr(array[i]) = base + i
LDR R4, R3                     ; load array[i] from memory

SUB R4, R4, R1                 ; array[i] = array[i] - 2

STR R3, R4                     ; store the result back into array[i]

RET                            ; end of kernel
