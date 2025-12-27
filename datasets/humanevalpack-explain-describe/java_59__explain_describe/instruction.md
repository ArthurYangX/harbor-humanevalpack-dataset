# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return the largest prime factor of n. Assume n > 1 and is not a prime.
    >>> largestPrimeFactor(13195)
    29
    >>> largestPrimeFactor(2048)
    2
     */
    public int largestPrimeFactor(int n) {
        int largest = 1;
        for (int j = 2; j <= n; j++) {
            if (n % j == 0) {
                boolean is_prime = j >= 2;
                for (int i = 2; i < j - 1; i++) {
                    if (j % i == 0) {
                        is_prime = false;
                        break;
                    }
                }
                if (is_prime) {
                    largest = Math.max(largest, j);
                }
            }
        }
        return largest;
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
    Return the largest prime factor of n. Assume n > 1 and is not a prime.
    >>> largestPrimeFactor(13195)
    29
    >>> largestPrimeFactor(2048)
    2
     */
    public int largestPrimeFactor(int n) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
