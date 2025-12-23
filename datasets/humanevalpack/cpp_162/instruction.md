# Problem Description

Write a C++ function `string string_to_md5(string text)` to solve the following problem:
Given a string 'text", return its md5 hash equivalent string.
If 'text" is an empty string, return None.
>>> string_to_md5("Hello world") == "3e25960a79dbc69b674cd4ec67a72c62"

# Prompt

/*
Given a string 'text", return its md5 hash equivalent string.
If 'text" is an empty string, return None.

>>> string_to_md5("Hello world") == "3e25960a79dbc69b674cd4ec67a72c62"
*/
#include<stdio.h>
#include<string>
#include<openssl/md5.h>
using namespace std;
string string_to_md5(string text){


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
