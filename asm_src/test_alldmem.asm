;;;;;;;;;;;;;
;
;
; basic gradient
;
; scored 2994 cycles
.threads 8

CONST R8, #255 ; max color
CONST R1, #1 ; spread

MUL R0, %blockIdx, %blockDim    ; Compute global thread index
ADD R0, R0, %threadIdx          ; R0 = thread ID
MUL R0, R0, R1 ; apply spread

CONST R5, #8 ; when this is equal to (number hardware threads * spread), it writes in clean lines
CONST R4, #1 ; inc tracker

CONST R7, #8  ; times to loop
CONST R6, #4  ; times to loop 2
CONST R2, #0  ; tracker
LOOP:
  CONST R10, #0 ; tracker 2
LOOP2:
    STR R0, R0 ; store current address
    ADD R0, R0, R5 ; inc addr
    ADD R10, R10, R4 ; inc tracker 2
    CMP R10, R6
    BRn LOOP2 ; loop if
  ADD R2, R2, R4 ; inc tracker
  CMP R2, R7
  BRn LOOP ; loop if R2 is negative compared to R7
RET
