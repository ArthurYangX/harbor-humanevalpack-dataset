# Problem Description

Write a Java function `public List<Integer> strangeSortList(List<Integer> lst)` to solve the following problem:
Given list of integers, return list in strange order.
Strange sorting, is when you start with the minimum value,
then maximum of the remaining integers, then minimum and so on.
Examples:
strangeSortList(Arrays.asList(1, 2, 3, 4)) == Arrays.asList(1, 4, 2, 3)
strangeSortList(Arrays.asList(5, 5, 5, 5)) == Arrays.asList(5, 5, 5, 5)
strangeSortList(Arrays.asList()) == Arrays.asList()

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given list of integers, return list in strange order.
    Strange sorting, is when you start with the minimum value,
    then maximum of the remaining integers, then minimum and so on.

    Examples:
    strangeSortList(Arrays.asList(1, 2, 3, 4)) == Arrays.asList(1, 4, 2, 3)
    strangeSortList(Arrays.asList(5, 5, 5, 5)) == Arrays.asList(5, 5, 5, 5)
    strangeSortList(Arrays.asList()) == Arrays.asList()
     */
    public List<Integer> strangeSortList(List<Integer> lst) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
