.threads 4
.data 0 0 0 0           ; Memory to store results

CONST R0, #5            ; 5
CONST R1, #3            ; Positive value to create -3
CONST R2, #0            ; Address to store results
STR R2, R1 ; store 3
RET ; End of program
;
;;; --- Subtraction 1: Positive - Negative ---
;SUB R3, R0, R1          ; R3 = 5 - (-3) = 8
;
;STR R2, R3
;ADD R2, R2, R12          ; store 8 and increment
;
;; --- Subtraction 2: Negative - Positive ---
;SUB R3, R1, R0          ; R3 = -3 - 5 = -8
;
;STR R2, R3
;ADD R2, R2, R12          ; store -8 (248) and increment
;
;; --- Subtraction 3: Negative - Negative ---
;SUB R3, R1, R1          ; R3 = -3 - (-3) = 0
;
;STR R2, R3
;ADD R2, R2, R12          ; store 0 and increment
;RET ; End of program
