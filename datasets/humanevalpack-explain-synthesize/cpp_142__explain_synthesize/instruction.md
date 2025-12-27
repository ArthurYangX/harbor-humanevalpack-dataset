# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
"
This function will take a vector of integers. For all entries in the vector, the function shall square the integer entry if its index is a 
multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
change the entries in the vector whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 

Examples:
For lst = {1,2,3} the output should be 6
For lst = {}  the output should be 0
For lst = {-1,-5,2,-1,-5}  the output should be -126
*/
#include<stdio.h>
#include<vector>
using namespace std;
int sum_squares(vector<int> lst){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
