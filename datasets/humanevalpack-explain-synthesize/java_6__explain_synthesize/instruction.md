# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in java that matches the explanation and passes the unit tests.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
    For each of the group, output the deepest level of nesting of parentheses.
    E.g. (()()) has maximum two levels of nesting while ((())) has three.

    >>> parseNestedParens("(()()) ((())) () ((())()())")
    [2, 3, 1, 3]
     */
    public List<Integer> parseNestedParens(String paren_string) {


# Instructions

Please write your solution in the file `solution/solution.java`.
Ensure your code is self-contained and runs correctly.
