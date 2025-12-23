# Problem Description

Write a Java function `public Map<String, Integer> histogram(String test)` to solve the following problem:
Given a string representing a space separated lowercase letters, return a dictionary
of the letter with the most repetition and containing the corresponding count.
If several letters have the same occurrence, return all of them.
Example:
histogram("a b c") == {"a": 1, "b": 1, "c": 1}
histogram("a b b a") == {"a": 2, "b": 2}
histogram("a b c a b") == {"a": 2, "b": 2}
histogram("b b b b a") == {"b": 4}
histogram("") == {}

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string representing a space separated lowercase letters, return a dictionary
    of the letter with the most repetition and containing the corresponding count.
    If several letters have the same occurrence, return all of them.

    Example:
    histogram("a b c") == {"a": 1, "b": 1, "c": 1}
    histogram("a b b a") == {"a": 2, "b": 2}
    histogram("a b c a b") == {"a": 2, "b": 2}
    histogram("b b b b a") == {"b": 4}
    histogram("") == {}
     */
    public Map<String, Integer> histogram(String test) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
