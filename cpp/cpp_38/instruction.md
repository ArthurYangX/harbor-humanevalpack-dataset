# Problem Description

Write a C++ function `string decode_cyclic(string s)` to solve the following problem:
takes as input string encoded with encode_cyclic function. Returns decoded string.

# Prompt

#include<stdio.h>
#include<string>
using namespace std;
string encode_cyclic(string s){ 
    // returns encoded string by cycling groups of three characters.  
    // split string to groups. Each of length 3.
    int l=s.length();
    int num=(l+2)/3;
    string x,output;
    int i;
    for (i=0;i*3<l;i++)
    {
        //cycle elements in each group. Unless group has fewer elements than 3.
        x=s.substr(i*3,3);
        if (x.length()==3) x=x.substr(1)+x[0];
        output=output+x;
    }
    return output;
}


string decode_cyclic(string s){ 
    /*
    takes as input string encoded with encode_cyclic function. Returns decoded string. 
    */


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
