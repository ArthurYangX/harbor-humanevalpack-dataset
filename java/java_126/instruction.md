# Problem Description

Write a Java function `public boolean isSorted(List<Integer> lst)` to solve the following problem:
Given a list of numbers, return whether or not they are sorted
in ascending order. If list has more than 1 duplicate of the same
number, return false. Assume no negative numbers and only integers.
Examples
isSorted(Arrays.asList(5)) -> true
isSorted(Arrays.asList(1, 2, 3, 4, 5)) -> true
isSorted(Arrays.asList(1, 3, 2, 4, 5)) -> false
isSorted(Arrays.asList(1, 2, 3, 4, 5, 6)) -> true
isSorted(Arrays.asList(1, 2, 3, 4, 5, 6, 7)) -> true
isSorted(Arrays.asList(1, 3, 2, 4, 5, 6, 7)) -> false
isSorted(Arrays.asList(1, 2, 2, 3, 3, 4)) -> true
isSorted(Arrays.asList(1, 2, 2, 2, 3, 4)) -> false

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a list of numbers, return whether or not they are sorted
    in ascending order. If list has more than 1 duplicate of the same
    number, return false. Assume no negative numbers and only integers.

    Examples
    isSorted(Arrays.asList(5)) -> true
    isSorted(Arrays.asList(1, 2, 3, 4, 5)) -> true
    isSorted(Arrays.asList(1, 3, 2, 4, 5)) -> false
    isSorted(Arrays.asList(1, 2, 3, 4, 5, 6)) -> true
    isSorted(Arrays.asList(1, 2, 3, 4, 5, 6, 7)) -> true
    isSorted(Arrays.asList(1, 3, 2, 4, 5, 6, 7)) -> false
    isSorted(Arrays.asList(1, 2, 2, 3, 3, 4)) -> true
    isSorted(Arrays.asList(1, 2, 2, 2, 3, 4)) -> false
     */
    public boolean isSorted(List<Integer> lst) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
