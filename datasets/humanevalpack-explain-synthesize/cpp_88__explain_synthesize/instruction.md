# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Given a vector of non-negative integers, return a copy of the given vector after sorting,
you will sort the given vector in ascending order if the sum( first index value, last index value) is odd,
or sort it in descending order if the sum( first index value, last index value) is even.

Note:
* don't change the given vector.

Examples:
* sort_vector({}) => {}
* sort_vector({5}) => {5}
* sort_vector({2, 4, 3, 0, 1, 5}) => {0, 1, 2, 3, 4, 5}
* sort_vector({2, 4, 3, 0, 1, 5, 6}) => {6, 5, 4, 3, 2, 1, 0}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> sort_array(vector<int> array){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
