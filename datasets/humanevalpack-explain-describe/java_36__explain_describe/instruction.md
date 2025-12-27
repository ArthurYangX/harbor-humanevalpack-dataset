# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return the number of times the digit 7 appears in integers less than n which are divisible by 11 or 13.
    >>> fizzBuzz(50)
    0
    >>> fizzBuzz(78)
    2
    >>> fizzBuzz(79)
    3
     */
    public int fizzBuzz(int n) {
        int result = 0;
        for (int i = 1; i < n; i++) {
            if (i % 11 == 0 || i % 13 == 0) {
                char[] digits = String.valueOf(i).toCharArray();
                for (char c : digits) {
                    if (c == '7') {
                        result += 1;
                    }
                }
            }
        }
        return result;
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
    Return the number of times the digit 7 appears in integers less than n which are divisible by 11 or 13.
    >>> fizzBuzz(50)
    0
    >>> fizzBuzz(78)
    2
    >>> fizzBuzz(79)
    3
     */
    public int fizzBuzz(int n) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
