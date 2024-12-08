# Assembled output for asm_src/reference_matmul.asm
0b0101000011011110, # MUL R0, %blockIdx, %blockDim   
0b0011000000001111, # ADD R0, R0, %threadIdx         ;  i = blockIdx * blockDim + threadIdx
0b1001000100000001, # CONST R1, #1                   ;  increment
0b1001001000000010, # CONST R2, #2                   ;  N (matrix inner dimension)
0b1001001100000000, # CONST R3, #0                   ;  baseA (matrix A base address)
0b1001010000000100, # CONST R4, #4                   ;  baseB (matrix B base address)
0b1001010100001000, # CONST R5, #8                   ;  baseC (matrix C base address)
0b0110011000000010, # DIV R6, R0, R2                 ;  row = i // N
0b0101011101100010, # MUL R7, R6, R2                 
0b0100011100000111, # SUB R7, R0, R7                 ;  col = i % N
0b1001100000000000, # CONST R8, #0                   ;  acc = 0
0b1001100100000000, # CONST R9, #0                   ;  k = 0
                    # LOOP:                          
0b0101101001100010, # MUL R10, R6, R2                
0b0011101010101001, # ADD R10, R10, R9               
0b0011101010100011, # ADD R10, R10, R3               ;  addr(A[i]) = row * N + k + baseA
0b0111101010100000, # LDR R10, R10                   ;  load A[i] from global memory
0b0101101110010010, # MUL R11, R9, R2                
0b0011101110110111, # ADD R11, R11, R7               
0b0011101110110100, # ADD R11, R11, R4               ;  addr(B[i]) = k * N + col + baseB
0b0111101110110000, # LDR R11, R11                   ;  load B[i] from global memory
0b0101110010101011, # MUL R12, R10, R11              
0b0011100010001100, # ADD R8, R8, R12                ;  acc = acc + A[i] * B[i]
0b0011100110010001, # ADD R9, R9, R1                 ;  increment k
0b0010000010010010, # CMP R9, R2                     
0b0001100000001100, # BRn LOOP                       ;  loop while k < N
0b0011100101010000, # ADD R9, R5, R0                 ;  addr(C[i]) = baseC + i
0b1000000010011000, # STR R9, R8                     ;  store C[i] in global memory
0b1111000000000000, # RET                            ;  end of kernel

# .data
1, 2, 3, 4,              #  matrix A (2 x 2)
1, 2, 3, 4               #  matrix B (2 x 2)

remember to specify thread count (4) in the testbench!
