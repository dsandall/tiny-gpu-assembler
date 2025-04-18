.threads 4
.data 0 0 0 0           ; Memory to store results

CONST R0, #5            ; First value (positive)
CONST R1, #3            ; Positive value to create -3
CONST R2, #0            ; Address to store results

; --- Create Negative Number ---
SUB R1, R2, R1          ; R1 = 0 - 3 = -3

; alternate super simple ending
STR R2, R1              ; Store result at memory[2]
RET                     ; End of program

;;; --- Subtraction 1: Positive - Negative ---
;SUB R3, R0, R1          ; R3 = 5 - (-3) = 8
;STR R2, R3              ; Store result at memory[0]
;CONST R12, #1
;ADD R2, R2, R12          ; Increment memory address
;
;; --- Subtraction 2: Negative - Positive ---
;SUB R3, R1, R0          ; R3 = -3 - 5 = -8
;STR R2, R3              ; Store result at memory[1]
;CONST R12, #1
;ADD R2, R2, R12          ; Increment memory address
;
;; --- Subtraction 3: Negative - Negative ---
;SUB R3, R1, R1          ; R3 = -3 - (-3) = 0
;STR R2, R3              ; Store result at memory[2]
;
;RET                     ; End of program
