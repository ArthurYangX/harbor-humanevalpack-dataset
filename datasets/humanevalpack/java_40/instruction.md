# Problem Description

Write a Java function `public boolean triplesSumToZero(List<Integer> l)` to solve the following problem:
triplesSumToZero takes a list of integers as an input.
it returns True if there are three distinct elements in the list that
sum to zero, and False otherwise.
>>> triplesSumToZero(Arrays.asList(1, 3, 5, 0))
false
>>> triplesSumToZero(Arrays.asList(1, 3, -2, 1))
true
>>> triplesSumToZero(Arrays.asList(1, 2, 3, 7))
false
>>> triplesSumToZero(Arrays.asList(2, 4, -5, 3, 9, 7))
true
>>> triplesSumToZero(Arrays.asList(1))
false

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    triplesSumToZero takes a list of integers as an input.
    it returns True if there are three distinct elements in the list that
    sum to zero, and False otherwise.

    >>> triplesSumToZero(Arrays.asList(1, 3, 5, 0))
    false
    >>> triplesSumToZero(Arrays.asList(1, 3, -2, 1))
    true
    >>> triplesSumToZero(Arrays.asList(1, 2, 3, 7))
    false
    >>> triplesSumToZero(Arrays.asList(2, 4, -5, 3, 9, 7))
    true
    >>> triplesSumToZero(Arrays.asList(1))
    false
     */
    public boolean triplesSumToZero(List<Integer> l) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
