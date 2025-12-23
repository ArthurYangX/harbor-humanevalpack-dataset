# Problem Description

Write a Java function `public Optional<Object> compareOne(Object a, Object b)` to solve the following problem:
Create a function that takes integers, floats, or strings representing
real numbers, and returns the larger variable in its given variable type.
Return None if the values are equal.
Note: If a real number is represented as a string, the floating point might be . or ,
compareOne(1, 2.5) -> Optional.of(2.5)
compareOne(1, "2,3") -> Optional.of("2,3")
compareOne("5,1", "6") -> Optional.of("6")
compareOne("1", 1) -> Optional.empty()

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Create a function that takes integers, floats, or strings representing
    real numbers, and returns the larger variable in its given variable type.
    Return None if the values are equal.
    Note: If a real number is represented as a string, the floating point might be . or ,

    compareOne(1, 2.5) -> Optional.of(2.5)
    compareOne(1, "2,3") -> Optional.of("2,3")
    compareOne("5,1", "6") -> Optional.of("6")
    compareOne("1", 1) -> Optional.empty()
     */
    public Optional<Object> compareOne(Object a, Object b) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
