# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Write a function that accepts two lists of strings and returns the list that has
// total number of chars in the all strings of the list less than the other list.
// 
// if the two lists have the same number of chars, return the first list.
// 
// Examples
// TotalMatch([], []) ➞ []
// TotalMatch(['hi', 'admin'], ['hI', 'Hi']) ➞ ['hI', 'Hi']
// TotalMatch(['hi', 'admin'], ['hi', 'hi', 'admin', 'project']) ➞ ['hi', 'admin']
// TotalMatch(['hi', 'admin'], ['hI', 'hi', 'hi']) ➞ ['hI', 'hi', 'hi']
// TotalMatch(['4'], ['1', '2', '3', '4', '5']) ➞ ['4']
func TotalMatch(lst1 []string,lst2 []string) []string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
