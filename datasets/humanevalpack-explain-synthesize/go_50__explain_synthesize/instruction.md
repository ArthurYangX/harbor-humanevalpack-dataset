# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

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

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
