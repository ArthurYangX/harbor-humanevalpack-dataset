# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Implement a function that takes an non-negative integer and returns a vector of the first n
integers that are prime numbers and less than n.
for example:
count_up_to(5) => {2,3}
count_up_to(11) => {2,3,5,7}
count_up_to(0) => {}
count_up_to(20) => {2,3,5,7,11,13,17,19}
count_up_to(1) => {}
count_up_to(18) => {2,3,5,7,11,13,17}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> count_up_to(int n){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
