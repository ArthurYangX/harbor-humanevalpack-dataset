# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Check if two words have the same characters.
    >>> sameChars("eabcdzzzz", "dddzzzzzzzddeddabc")
    true
    >>> sameChars("abcd", "dddddddabc")
    true
    >>> sameChars("dddddddabc", "abcd")
    true
    >>> sameChars("eabcd", "dddddddabc")
    false
    >>> sameChars("abcd", "dddddddabce")
    false
    >>> sameChars("eabcdzzzz", "dddzzzzzzzddddabc")
    false
     */
    public boolean sameChars(String s0, String s1) {
        Set<Character> set0 = new HashSet<>();
        for (char c : s0.toCharArray()) {
            set0.add(c);
        }
        Set<Character> set1 = new HashSet<>();
        for (char c : s1.toCharArray()) {
            set1.add(c);
        }
        return set0.equals(set1);
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
    Check if two words have the same characters.
    >>> sameChars("eabcdzzzz", "dddzzzzzzzddeddabc")
    true
    >>> sameChars("abcd", "dddddddabc")
    true
    >>> sameChars("dddddddabc", "abcd")
    true
    >>> sameChars("eabcd", "dddddddabc")
    false
    >>> sameChars("abcd", "dddddddabce")
    false
    >>> sameChars("eabcdzzzz", "dddzzzzzzzddddabc")
    false
     */
    public boolean sameChars(String s0, String s1) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
