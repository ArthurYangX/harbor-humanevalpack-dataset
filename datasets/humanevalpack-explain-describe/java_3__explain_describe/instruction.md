# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    You're given a list of deposit and withdrawal operations on a bank account that starts with
    zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
    at that point function should return True. Otherwise it should return False.
    >>> belowZero(Arrays.asList(1, 2, 3))
    false
    >>> belowZero(Arrays.asList(1, 2, -4, 5))
    true
     */
    public boolean belowZero(List<Integer> operations) {
        int balance = 0;

        for (int op : operations) {
            balance += op;
            if (balance < 0) {
                return true;
            }
        }

        return false;
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
    You're given a list of deposit and withdrawal operations on a bank account that starts with
    zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
    at that point function should return True. Otherwise it should return False.
    >>> belowZero(Arrays.asList(1, 2, 3))
    false
    >>> belowZero(Arrays.asList(1, 2, -4, 5))
    true
     */
    public boolean belowZero(List<Integer> operations) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
