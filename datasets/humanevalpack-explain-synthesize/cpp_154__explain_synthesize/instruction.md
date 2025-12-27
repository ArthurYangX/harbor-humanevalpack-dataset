# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
You are given 2 words. You need to return true if the second word or any of its rotations is a substring in the first word
cycpattern_check("abcd","abd") => false
cycpattern_check("hello","ell") => true
cycpattern_check("whassup","psus") => false
cycpattern_check("abab","baa") => true
cycpattern_check("efef","eeff") => false
cycpattern_check("himenss",'simen") => true

*/
#include<stdio.h>
#include<string>
using namespace std;
bool cycpattern_check(string a,string b){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
