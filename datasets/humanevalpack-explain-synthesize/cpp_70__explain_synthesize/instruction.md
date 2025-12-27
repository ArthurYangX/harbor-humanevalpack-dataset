# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Given vector of integers, return vector in strange order.
Strange sorting, is when you start with the minimum value,
then maximum of the remaining integers, then minimum and so on.

Examples:
strange_sort_vector({1, 2, 3, 4}) == {1, 4, 2, 3}
strange_sort_vector({5, 5, 5, 5}) == {5, 5, 5, 5}
strange_sort_vector({}) == {}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> strange_sort_list(vector<int> lst){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
