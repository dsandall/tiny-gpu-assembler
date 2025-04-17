# Assembled output for asm_src/test_negatives.asm
0b1001000000000101, # CONST R0, #5                   ;  5
0b1001000100000011, # CONST R1, #3                   ;  Positive value to create -3
0b1001001000000000, # CONST R2, #0                   ;  Address to store results
0b1000000000100001, # STR R2, R1                     ;  store 3
0b1111000000000000, # RET                            ;  End of program

# .data
0, 0, 0, 0               #  Memory to store results

remember to specify thread count (4) in the testbench!
