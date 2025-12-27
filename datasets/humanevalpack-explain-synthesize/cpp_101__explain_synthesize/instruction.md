# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
You will be given a string of words separated by commas or spaces. Your task is
to split the string into words and return a vector of the words.

For example:
words_string("Hi, my name is John") == {"Hi", "my", "name", "is", "John"}
words_string("One, two, three, four, five, six") == {"One", 'two", 'three", "four", "five", 'six"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> words_string(string s){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
