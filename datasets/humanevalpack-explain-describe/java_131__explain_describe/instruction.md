# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a positive integer n, return the product of the odd digits.
    Return 0 if all digits are even.
    For example:
    digits(1)  == 1
    digits(4)  == 0
    digits(235) == 15
     */
    public int digits(int n) {
        int product = 1, odd_count = 0;
        for (char digit : String.valueOf(n).toCharArray()) {
            int int_digit = digit - '0';
            if (int_digit % 2 == 1) {
                product *= int_digit;
                odd_count += 1;
            }
        }
        if (odd_count == 0) {
            return 0;
        } else {
            return product;
        }
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
    Given a positive integer n, return the product of the odd digits.
    Return 0 if all digits are even.
    For example:
    digits(1)  == 1
    digits(4)  == 0
    digits(235) == 15
     */
    public int digits(int n) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
