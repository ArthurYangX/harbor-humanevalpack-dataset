# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Write a function that takes a string and returns an ordered version of it.
Ordered version of string, is a string where all words (separated by space)
are replaced by a new word where all the characters arranged in
ascending order based on ascii value.
Note: You should keep the order of words and blank spaces in the sentence.

For example:
anti_shuffle("Hi") returns "Hi"
anti_shuffle("hello") returns "ehllo"
anti_shuffle("Hello World!!!") returns "Hello !!!Wdlor"
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string anti_shuffle(string s){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
