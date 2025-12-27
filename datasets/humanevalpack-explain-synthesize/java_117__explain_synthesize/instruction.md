# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in java that matches the explanation and passes the unit tests.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string s and a natural number n, you have been tasked to implement
    a function that returns a list of all words from string s that contain exactly
    n consonants, in order these words appear in the string s.
    If the string s is empty then the function should return an empty list.
    Note: you may assume the input string contains only letters and spaces.
    Examples:
    selectWords("Mary had a little lamb", 4) ==> ["little"]
    selectWords("Mary had a little lamb", 3) ==> ["Mary", "lamb"]
    selectWords("simple white space", 2) ==> []
    selectWords("Hello world", 4) ==> ["world"]
    selectWords("Uncle sam", 3) ==> ["Uncle"]
     */
    public List<String> selectWords(String s, int n) {


# Instructions

Please write your solution in the file `solution/solution.java`.
Ensure your code is self-contained and runs correctly.
