# Problem Description

Write a Java function `public boolean belowZero(List<Integer> operations)` to solve the following problem:
You're given a list of deposit and withdrawal operations on a bank account that starts with
zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
at that point function should return True. Otherwise it should return False.
>>> belowZero(Arrays.asList(1, 2, 3))
false
>>> belowZero(Arrays.asList(1, 2, -4, 5))
true

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    You're given a list of deposit and withdrawal operations on a bank account that starts with
    zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
    at that point function should return True. Otherwise it should return False.
    >>> belowZero(Arrays.asList(1, 2, 3))
    false
    >>> belowZero(Arrays.asList(1, 2, -4, 5))
    true
     */
    public boolean belowZero(List<Integer> operations) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
