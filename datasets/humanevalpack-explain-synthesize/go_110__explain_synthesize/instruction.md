# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// In this problem, you will implement a function that takes two lists of numbers,
// and determines whether it is possible to perform an Exchange of elements
// between them to make lst1 a list of only even numbers.
// There is no limit on the number of Exchanged elements between lst1 and lst2.
// If it is possible to Exchange elements between the lst1 and lst2 to make
// all the elements of lst1 to be even, return "YES".
// Otherwise, return "NO".
// For example:
// Exchange([1, 2, 3, 4], [1, 2, 3, 4]) => "YES"
// Exchange([1, 2, 3, 4], [1, 5, 3, 4]) => "NO"
// It is assumed that the input lists will be non-empty.
func Exchange(lst1, lst2 []int) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
