# Problem Description

Write a Java function `public boolean pairsSumToZero(List<Integer> l)` to solve the following problem:
pairsSumToZero takes a list of integers as an input.
it returns True if there are two distinct elements in the list that
sum to zero, and False otherwise.
>>> pairsSumToZero(Arrays.asList(1, 3, 5, 0))
false
>>> pairsSumToZero(Arrays.asList(1, 3, -2, 1))
false
>>> pairsSumToZero(Arrays.asList(1, 2, 3, 7))
false
>>> pairsSumToZero(Arrays.asList(2, 4, -5, 3, 5, 7))
true
>>> pairsSumToZero(Arrays.asList(1))
false

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    pairsSumToZero takes a list of integers as an input.
    it returns True if there are two distinct elements in the list that
    sum to zero, and False otherwise.
    >>> pairsSumToZero(Arrays.asList(1, 3, 5, 0))
    false
    >>> pairsSumToZero(Arrays.asList(1, 3, -2, 1))
    false
    >>> pairsSumToZero(Arrays.asList(1, 2, 3, 7))
    false
    >>> pairsSumToZero(Arrays.asList(2, 4, -5, 3, 5, 7))
    true
    >>> pairsSumToZero(Arrays.asList(1))
    false
     */
    public boolean pairsSumToZero(List<Integer> l) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
