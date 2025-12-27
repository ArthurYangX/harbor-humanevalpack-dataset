# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

/*
Given a string s and a natural number n, you have been tasked to implement 
a function that returns a vector of all words from string s that contain exactly 
n consonants, in order these words appear in the string s.
If the string s is empty then the function should return an empty vector.
Note: you may assume the input string contains only letters and spaces.
Examples:
select_words("Mary had a little lamb", 4) ==> {"little"}
select_words("Mary had a little lamb", 3) ==> {"Mary", "lamb"}
select_words('simple white space", 2) ==> {}
select_words("Hello world", 4) ==> {"world"}
select_words("Uncle sam", 3) ==> {"Uncle"}
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
vector<string> select_words(string s,int n){


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
