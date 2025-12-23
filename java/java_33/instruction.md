# Problem Description

Write a Java function `public List<Integer> sortThird(List<Integer> l)` to solve the following problem:
This function takes a list l and returns a list l' such that
l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
to the values of the corresponding indicies of l, but sorted.
>>> sortThird(Arrays.asList(1, 2, 3))
[1, 2, 3]
>>> sortThird(Arrays.asList(5, 6, 3, 4, 8, 9, 2))
[2, 6, 3, 4, 8, 9, 5]

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    This function takes a list l and returns a list l' such that
    l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
    to the values of the corresponding indicies of l, but sorted.
    >>> sortThird(Arrays.asList(1, 2, 3))
    [1, 2, 3]
    >>> sortThird(Arrays.asList(5, 6, 3, 4, 8, 9, 2))
    [2, 6, 3, 4, 8, 9, 5]
     */
    public List<Integer> sortThird(List<Integer> l) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
