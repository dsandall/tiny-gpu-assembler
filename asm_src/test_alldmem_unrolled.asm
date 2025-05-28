;;;;;;;;;;;;;
;
;
; scored 2662 cycles
;
.threads 8
;CONST R5, #1 ; #threads

CONST R1, #8 ; #spread
MUL R0, %blockIdx, %blockDim    ; Compute global thread index
ADD R0, R0, %threadIdx          ; R0 = thread ID
MUL R0, R0, R1 ; apply spread

CONST R2, #0  ; tracker

LOOP: 
  CONST R1, #1 ;
  ADD R2, R2, R1 ; inc tracker

;  STR R0, R0 ; store current
;  ADD R0, R0, R1 ; inc addr
;  STR R0, R0 ; store current; 8x unrolled
;  ADD R0, R0, R1 ; inc addr ; 8x unrolled
;  STR R0, R0 ; store current
;  ADD R0, R0, R1 ; inc addr
;  STR R0, R0 ; store current; 8x unrolled
;  ADD R0, R0, R1 ; inc addr ; 8x unrolled
;  STR R0, R0 ; store current
;  ADD R0, R0, R1 ; inc addr
;  STR R0, R0 ; store current; 8x unrolled
;  ADD R0, R0, R1 ; inc addr ; 8x unrolled
;  STR R0, R0 ; store current
;  ADD R0, R0, R1 ; inc addr
;  STR R0, R0 ; store current; 8x unrolled
;  ADD R0, R0, R1 ; inc addr ; 8x unrolled
  STR R0, R0 ; store current
  ADD R0, R0, R1 ; inc addr
  STR R0, R0 ; store current; 8x unrolled
  ADD R0, R0, R1 ; inc addr ; 8x unrolled
  STR R0, R0 ; store current
  ADD R0, R0, R1 ; inc addr
  STR R0, R0 ; store current; 8x unrolled
  ADD R0, R0, R1 ; inc addr ; 8x unrolled
  STR R0, R0 ; store current
  ADD R0, R0, R1 ; inc addr
  STR R0, R0 ; store current; 8x unrolled
  ADD R0, R0, R1 ; inc addr ; 8x unrolled
  STR R0, R0 ; store current
  ADD R0, R0, R1 ; inc addr
  STR R0, R0 ; store current; 8x unrolled
  ADD R0, R0, R1 ; inc addr ; 8x unrolled

  ;CONST R1, #48 ; 16*(4-1)
  CONST R1, #56 ; 8*(8-1)
  ADD R0, R0, R1 ;

  CONST R1, #4  ; #times to loop
  CMP R2, R1
  BRn LOOP ; loop if R2 is negative compared to R7
RET


