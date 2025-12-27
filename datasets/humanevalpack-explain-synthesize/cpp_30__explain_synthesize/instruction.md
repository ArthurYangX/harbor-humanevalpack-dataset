# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Return only positive numbers in the vector.
>>> get_positive({-1, 2, -4, 5, 6})
{2, 5, 6}
>>> get_positive({5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10})
{5, 3, 2, 3, 9, 123, 1}
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
vector<float> get_positive(vector<float> l){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
