# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Given a vector of numbers, return whether or not they are sorted
in ascending order. If vector has more than 1 duplicate of the same
number, return false. Assume no negative numbers and only integers.

Examples
is_sorted({5}) ➞ true
is_sorted({1, 2, 3, 4, 5}) ➞ true
is_sorted({1, 3, 2, 4, 5}) ➞ false
is_sorted({1, 2, 3, 4, 5, 6}) ➞ true
is_sorted({1, 2, 3, 4, 5, 6, 7}) ➞ true
is_sorted({1, 3, 2, 4, 5, 6, 7}) ➞ false
is_sorted({1, 2, 2, 3, 3, 4}) ➞ true
is_sorted({1, 2, 2, 2, 3, 4}) ➞ false
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
bool is_sorted(vector<int> lst){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
