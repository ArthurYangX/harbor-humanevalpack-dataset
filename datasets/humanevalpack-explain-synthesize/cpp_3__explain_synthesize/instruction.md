# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
You"re given a vector of deposit and withdrawal operations on a bank account that starts with
zero balance. Your task is to detect if at any point the balance of account falls below zero, and
at that point function should return true. Otherwise it should return false.
>>> below_zero({1, 2, 3})
false
>>> below_zero({1, 2, -4, 5})
true
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool below_zero(vector<int> operations){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
