# tiny-gpu-assembler
### A bare minimum assembler for the TinyGPU ISA

# Usage:
- install rust and cargo
- ``cargo run [source.asm] > [output.py.asm]
- make sure to test any generated code with a sensible test case using the CocoTB simulator

# Repository Contents
- src/
    - source files for building the assembler
- asm_src/
    - reference assembly programs that should both compile and run properly on the TinyGPU 
- target/  (not included, run ``cargo build``)
    - build directory, target specific 

# Features
- Label and branching support
- Limited error detection, syntax checking
- Exports Machine Code, Source Code, and comments, line by line, in a Python and CocoTB compatible format for easy integration with the TinyGPU test environment  

# Future Improvements
- Pseudoinstructions
- Register Renaming (improves source readability)
- Shared Memory Dependency Detection 
    - would be VERY valuable for writing cache optimized code
    - a compiler/assembler that has an awareness of cache and thread limitations could pick up some of the slack and improve performance in some scenarios, rather than expecting the hardware to solve all hazards
