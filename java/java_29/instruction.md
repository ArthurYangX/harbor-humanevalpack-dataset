# Problem Description

Write a Java function `public List<String> filterByPrefix(List<String> strings, String prefix)` to solve the following problem:
Filter an input list of strings only for ones that start with a given prefix.
>>> filterByPrefix(List.of(), "a")
[]
>>> filterByPrefix(Arrays.asList("abc", "bcd", "cde", "array"), "a")
["abc", "array"]

# Prompt

import java.util.*;
import java.lang.*;
import java.util.stream.Collectors;

class Solution {
    /**
    Filter an input list of strings only for ones that start with a given prefix.
    >>> filterByPrefix(List.of(), "a")
    []
    >>> filterByPrefix(Arrays.asList("abc", "bcd", "cde", "array"), "a")
    ["abc", "array"]
     */
    public List<String> filterByPrefix(List<String> strings, String prefix) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
