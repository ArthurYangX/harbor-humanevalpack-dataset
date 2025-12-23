# Problem Description

Write a Java function `public String decodeCyclic(String s)` to solve the following problem:
takes as input string encoded with encodeCyclic function. Returns decoded string.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    returns encoded string by cycling groups of three characters.
     */
    public String encodeCyclic(String s) {
        // split string to groups. Each of length 3.
        List<String> groups = new ArrayList<>();
        for (int i = 0; i < s.length(); i += 3) {
            groups.add(s.substring(i, Math.min(i + 3, s.length())));
        }
        // cycle elements in each group. Unless group has fewer elements than 3.
        for (int i = 0; i < groups.size(); i++) {
            if (groups.get(i).length() == 3) {
                groups.set(i, groups.get(i).substring(1) + groups.get(i).charAt(0));
            }
        }
        return String.join("", groups);
    }

    /**
    takes as input string encoded with encodeCyclic function. Returns decoded string.
     */
    public String decodeCyclic(String s) {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
