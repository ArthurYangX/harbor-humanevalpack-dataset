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
    returns encoded string by shifting every character by 5 in the alphabet.
     */
    public String encodeShift(String s) {
        StringBuilder sb = new StringBuilder();
        for (char ch : s.toCharArray()) {
            sb.append((char) ('a' + ((ch + 5 - 'a') % 26)));
        }
        return sb.toString();
    }

    /**
    takes as input string encoded with encodeShift function. Returns decoded string.
     */
    public String decodeShift(String s) {


# Instructions

Please write your solution in the file `solution/solution.java`.
Ensure your code is self-contained and runs correctly.
