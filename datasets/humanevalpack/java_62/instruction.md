# Problem Description

Write a Java function `public List<Integer> derivative(List<Integer> xs)` to solve the following problem:
xs represent coefficients of a polynomial.
xs[0] + xs[1] * x + xs[2] * x^2 + ....
Return derivative of this polynomial in the same form.
>>> derivative(Arrays.asList(3, 1, 2, 4, 5))
[1, 4, 12, 20]
>>> derivative(Arrays.asList(1, 2, 3]))
[2, 6]

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    xs represent coefficients of a polynomial.
    xs[0] + xs[1] * x + xs[2] * x^2 + ....
     Return derivative of this polynomial in the same form.
    >>> derivative(Arrays.asList(3, 1, 2, 4, 5))
    [1, 4, 12, 20]
    >>> derivative(Arrays.asList(1, 2, 3]))
    [2, 6]
     */
    public List<Integer> derivative(List<Integer> xs) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
