# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Task
    We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
    then check if the result string is palindrome.
    A string is called palindrome if it reads the same backward as forward.
    You should return a tuple containing the result string and true/false for the check.
    Example
    For s = "abcde", c = "ae", the result should be ("bcd",false)
    For s = "abcdef", c = "b"  the result should be ("acdef",false)
    For s = "abcdedcba", c = "ab", the result should be ("cdedc",true)
     */
    public List<Object> reverseDelete(String s, String c) {
        StringBuilder sb = new StringBuilder();
        for (char ch : s.toCharArray()) {
            if (c.indexOf(ch) == -1) {
                sb.append(ch);
            }
        }
        return Arrays.asList(sb.toString(), sb.toString().equals(sb.reverse().toString()));
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
    Task
    We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
    then check if the result string is palindrome.
    A string is called palindrome if it reads the same backward as forward.
    You should return a tuple containing the result string and true/false for the check.
    Example
    For s = "abcde", c = "ae", the result should be ("bcd",false)
    For s = "abcdef", c = "b"  the result should be ("acdef",false)
    For s = "abcdedcba", c = "ab", the result should be ("cdedc",true)
     */
    public List<Object> reverseDelete(String s, String c) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
