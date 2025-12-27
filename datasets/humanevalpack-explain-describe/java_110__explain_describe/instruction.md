# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    In this problem, you will implement a function that takes two lists of numbers,
    and determines whether it is possible to perform an exchange of elements
    between them to make lst1 a list of only even numbers.
    There is no limit on the number of exchanged elements between lst1 and lst2.
    If it is possible to exchange elements between the lst1 and lst2 to make
    all the elements of lst1 to be even, return "YES".
    Otherwise, return "NO".
    For example:
    exchange(Arrays.asList(1, 2, 3, 4), Arrays.asList(1, 2, 3, 4)) => "YES"
    exchange(Arrays.asList(1, 2, 3, 4), Arrays.asList(1, 5, 3, 4)) => "NO"
    It is assumed that the input lists will be non-empty.
     */
    public String exchange(List<Integer> lst1, List<Integer> lst2) {
        int odd = 0, even = 0;
        for (int i : lst1) {
            if (i % 2 == 1) {
                odd += 1;
            }
        }
        for (int i : lst2) {
            if (i % 2 == 0) {
                even += 1;
            }
        }
        if (even >= odd) {
            return "YES";
        }
        return "NO";
    }
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    In this problem, you will implement a function that takes two lists of numbers,
    and determines whether it is possible to perform an exchange of elements
    between them to make lst1 a list of only even numbers.
    There is no limit on the number of exchanged elements between lst1 and lst2.
    If it is possible to exchange elements between the lst1 and lst2 to make
    all the elements of lst1 to be even, return "YES".
    Otherwise, return "NO".
    For example:
    exchange(Arrays.asList(1, 2, 3, 4), Arrays.asList(1, 2, 3, 4)) => "YES"
    exchange(Arrays.asList(1, 2, 3, 4), Arrays.asList(1, 5, 3, 4)) => "NO"
    It is assumed that the input lists will be non-empty.
     */
    public String exchange(List<Integer> lst1, List<Integer> lst2) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
