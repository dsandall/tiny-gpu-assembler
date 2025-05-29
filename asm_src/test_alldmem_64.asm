;;;;;;;;;;;;;
;
;
; Super slow because it relies on dispatcher 8x
; (not good, but high thread count)
; scored 2994 cycles
;
.threads 64
;CONST R5, #1 ; #threads
CONST R8, #255 ; max color
CONST R1, #4 ; #spread
MUL R0, %blockIdx, %blockDim    ; Compute global thread index
ADD R0, R0, %threadIdx          ; R0 = thread ID

MUL R0, R0, R1 ; apply spread

CONST R2, #0  ; tracker
LOOP: 
  CONST R10, #0 ; tracker 2
LOOP2:
    CONST R1, #1 ;
    STR R0, R0 ; store current
    ADD R0, R0, R1 ; inc addr
    STR R0, R0 ; store current; 4x unrolled
    ADD R0, R0, R1 ; inc addr ; 4x unrolled
    STR R0, R0 ; store current
    ADD R0, R0, R1 ; inc addr
    STR R0, R0 ; store current; 4x unrolled
    ADD R0, R0, R1 ; inc addr ; 4x unrolled
    
    CONST R1, #1 ;
    ADD R10, R10, R1 ; inc tracker 2

    CONST R1, #1  ; #times to loop
    CMP R10, R1
    BRn LOOP2 ; loop if
  
  CONST R1, #1 ;
  ADD R2, R2, R1 ; inc tracker

  CONST R1, #64 ; 4*8
  ADD R0, R0, R1 ;

  CONST R1, #1  ; #times to loop
  CMP R2, R1
  BRn LOOP ; loop if R2 is negative compared to R7
RET


