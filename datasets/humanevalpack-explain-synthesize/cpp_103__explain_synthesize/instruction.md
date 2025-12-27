# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
You are given two positive integers n and m, and your task is to compute the
average of the integers from n through m (including n and m). 
Round the answer to the nearest integer(smaller one) and convert that to binary.
If n is greater than m, return "-1".
Example:
rounded_avg(1, 5) => "11"
rounded_avg(7, 5) => "-1"
rounded_avg(10, 20) => "1111"
rounded_avg(20, 33) => "11010"
*/
#include<stdio.h>
#include<math.h>
#include<string>
using namespace std;
string rounded_avg(int n,int m){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
