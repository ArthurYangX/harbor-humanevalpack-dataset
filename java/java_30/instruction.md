# Problem Description

Write a Java function `public List<Integer> getPositive(List<Integer> l)` to solve the following problem:
Return only positive numbers in the list.
>>> getPositive(Arrays.asList(-1, 2, -4, 5, 6))
[2, 5, 6]
>>> getPositive(Arrays.asList(5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10))
[5, 3, 2, 3, 9, 123, 1]

# Prompt

import java.util.*;
import java.lang.*;
import java.util.stream.Collectors;

class Solution {
    /**
    Return only positive numbers in the list.
    >>> getPositive(Arrays.asList(-1, 2, -4, 5, 6))
    [2, 5, 6]
    >>> getPositive(Arrays.asList(5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10))
    [5, 3, 2, 3, 9, 123, 1]
     */
    public List<Integer> getPositive(List<Integer> l) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
