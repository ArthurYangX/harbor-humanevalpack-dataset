# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Input to this function is a string representing musical notes in a special ASCII format.
// Your task is to parse this string and return list of integers corresponding to how many beats does each
// not last.
// 
// Here is a legend:
// 'o' - whole note, lasts four beats
// 'o|' - half note, lasts two beats
// '.|' - quater note, lasts one beat
// 
// >>> ParseMusic('o o| .| o| o| .| .| .| .| o o')
// [4, 2, 1, 2, 2, 1, 1, 1, 1, 4, 4]
func ParseMusic(music_string string) []int{


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
