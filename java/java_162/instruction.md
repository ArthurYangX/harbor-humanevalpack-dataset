# Problem Description

Write a Java function `public Optional<String> stringToMd5(String text) throws NoSuchAlgorithmException` to solve the following problem:
Given a string "text", return its md5 hash equivalent string with length being 32.
If "text" is an empty string, return Optional.empty().
>>> stringToMd5("Hello world") == "3e25960a79dbc69b674cd4ec67a72c62"

# Prompt

import java.math.BigInteger;
import java.security.*;
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string "text", return its md5 hash equivalent string with length being 32.
    If "text" is an empty string, return Optional.empty().
    
    >>> stringToMd5("Hello world") == "3e25960a79dbc69b674cd4ec67a72c62"
     */
    public Optional<String> stringToMd5(String text) throws NoSuchAlgorithmException {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
