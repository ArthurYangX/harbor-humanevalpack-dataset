# Problem Description

Write a Java function `public List<Integer> filterIntegers(List<Object> values)` to solve the following problem:
Filter given list of any values only for integers
>>> filter_integers(Arrays.asList('a', 3.14, 5))
[5]
>>> filter_integers(Arrays.asList(1, 2, 3, "abc", Map.of(), List.of()))
[1, 2, 3]

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Filter given list of any values only for integers
    >>> filter_integers(Arrays.asList('a', 3.14, 5))
    [5]
    >>> filter_integers(Arrays.asList(1, 2, 3, "abc", Map.of(), List.of()))
    [1, 2, 3]
     */
    public List<Integer> filterIntegers(List<Object> values) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
