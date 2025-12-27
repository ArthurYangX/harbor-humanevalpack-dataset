# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
This function takes a vector l and returns a vector l' such that
l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
to the values of the corresponding indicies of l, but sorted.
>>> sort_third({1, 2, 3})
{1, 2, 3}
>>> sort_third({5, 6, 3, 4, 8, 9, 2})
{2, 6, 3, 4, 8, 9, 5}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> sort_third(vector<int> l){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
