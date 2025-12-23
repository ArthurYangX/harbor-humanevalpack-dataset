# Problem Description

Write a Java function `public List<String> filterBySubstring(List<String> strings, String substring)` to solve the following problem:
Filter an input list of strings only for ones that contain given substring
>>> filterBySubstring(List.of(), "a")
[]
>>> filterBySubstring(Arrays.asList("abc", "bacd", "cde", "array"), "a")
["abc", "bacd", "array"]

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Filter an input list of strings only for ones that contain given substring
    >>> filterBySubstring(List.of(), "a")
    []
    >>> filterBySubstring(Arrays.asList("abc", "bacd", "cde", "array"), "a")
    ["abc", "bacd", "array"]
     */
    public List<String> filterBySubstring(List<String> strings, String substring) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
