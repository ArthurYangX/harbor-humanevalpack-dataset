# Problem Description

Write a Go function `func StringToMd5(text string) interface{}` to solve the following problem:
Given a string 'text', return its md5 hash equivalent string.
If 'text' is an empty string, return nil.
>>> StringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'

# Prompt

import (
    "crypto/md5"
    "fmt"
)

// Given a string 'text', return its md5 hash equivalent string.
// If 'text' is an empty string, return nil.
// 
// >>> StringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
func StringToMd5(text string) interface{} {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
