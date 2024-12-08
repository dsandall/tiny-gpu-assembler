# Assembled output for asm_src/test_load.asm
0b1001000100000100, # CONST R1, #4                   ;  Increment for addresses (4 bytes later)
0b1001001000000100, # CONST R2, #4                   ;  Number of iterations
0b1001001100000000, # CONST R3, #0                   ;  Base address
0b1001010000000010, # CONST R4, #2                   ;  Multiply factor (2 for doubling)
0b0101000011011110, # MUL R0, %blockIdx, %blockDim   ;  Compute global thread index
0b0011000000001111, # ADD R0, R0, %threadIdx         ;  R0 = thread ID
0b0011011000110000, # ADD R6, R3, R0                 ;  Initial address = base + thread offset
0b1001011100000000, # CONST R7, #0                   ;  Iteration counter
                    # LOOP:                          
0b0111100001100000, # LDR R8, R6                     ;  R8 = memory[R6] (load)
0b0101100010000100, # MUL R8, R8, R4                 ;  R8 = R8 * 2
0b0011011001100001, # ADD R6, R6, R1                 ;  R6 = R6 + 4 (next store address)
0b1000000001101000, # STR R6, R8                     ;  memory[R6] = R8 (store)
0b0000000000000000, # NOP                            
0b0000000000000000, # NOP                            
0b1001110000000001, # CONST R12, #1                  
0b0011011101111100, # ADD R7, R7, R12                ;  R7 = R7 + 1
0b0010000001110010, # CMP R7, R2                     
0b0001100000001000, # BRn LOOP                       ;  Branch back to LOOP if R7 < 4
0b1111000000000000, # RET                            ;  End of program

# .data
1, 2, 3, 4               #  Initial data in memory (4 bytes)

remember to specify thread count (4) in the testbench!
