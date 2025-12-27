# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in cpp that matches the explanation and passes the unit tests.

# Prompt

#include<stdio.h>
#include<string>
using namespace std;
string encode_shift(string s){
    // returns encoded string by shifting every character by 5 in the alphabet.
    string out;
    int i;
    for (i=0;i<s.length();i++)
    {
        int w=((int)s[i]+5-(int)'a')%26+(int)'a';   
        out=out+(char)w;
    }
    return out;
}
string decode_shift(string s){
    // takes as input string encoded with encode_shift function. Returns decoded string.


# Instructions

Please write your solution in the file `solution/solution.cpp`.
Ensure your code is self-contained and runs correctly.
