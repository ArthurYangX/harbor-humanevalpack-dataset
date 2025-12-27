# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Given a positive floating point number, it can be decomposed into
and integer part (largest integer smaller than given number) and decimals
(leftover part always smaller than 1).

Return the decimal part of the number.
>>> truncate_number(3.5)
0.5
*/
#include<stdio.h>
#include<math.h>
using namespace std;
float truncate_number(float number){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
