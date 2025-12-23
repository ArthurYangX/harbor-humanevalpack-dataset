# Problem Description

Write a Go function `func DecodeShift(s string) string` to solve the following problem:
takes as input string encoded with EncodeShift function. Returns decoded string.

# Prompt

// returns encoded string by shifting every character by 5 in the alphabet.
func EncodeShift(s string) string {
    runes := []rune(s)
    newRunes := make([]rune, 0)
    for _, ch := range runes {
        newRunes = append(newRunes, (ch+5-'a')%26+'a')
    }
    return string(runes)
}

// takes as input string encoded with EncodeShift function. Returns decoded string.
func DecodeShift(s string) string {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
