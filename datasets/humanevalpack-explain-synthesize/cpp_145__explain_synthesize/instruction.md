# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Write a function which sorts the given vector of integers
in ascending order according to the sum of their digits.
Note: if there are several items with similar sum of their digits,
order them based on their index in original vector.

For example:
>>> order_by_points({1, 11, -1, -11, -12}) == {-1, -11, 1, -12, 11}
>>> order_by_points({}) == {}
*/
#include<stdio.h>
#include<math.h>
#include<vector>
#include<string>
using namespace std;
vector<int> order_by_points(vector<int> nums){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
