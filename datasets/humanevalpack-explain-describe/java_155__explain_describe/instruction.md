# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given an integer. return a tuple that has the number of even and odd digits respectively.
    
     Example:
        evenOddCount(-12) ==> (1, 1)
        evenOddCount(123) ==> (1, 2)
     */
    public List<Integer> evenOddCount(int num) {
        int even_count = 0, odd_count = 0;
        for (char i : String.valueOf(Math.abs(num)).toCharArray()) {
            if ((i - '0') % 2 == 0) {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
        return Arrays.asList(even_count, odd_count);
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
    Given an integer. return a tuple that has the number of even and odd digits respectively.
    
     Example:
        evenOddCount(-12) ==> (1, 1)
        evenOddCount(123) ==> (1, 2)
     */
    public List<Integer> evenOddCount(int num) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
