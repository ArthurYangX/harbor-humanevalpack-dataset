# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Write a function that returns True if the object q will fly, and False otherwise.
    The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.

    Example:
    willItFly(Arrays.asList(1, 2), 5) -> false
    # 1+2 is less than the maximum possible weight, but it's unbalanced.

    willItFly(Arrays.asList(3, 2, 3), 1) -> false
    # it's balanced, but 3+2+3 is more than the maximum possible weight.

    willItFly(Arrays.asList(3, 2, 3), 9) -> true
    # 3+2+3 is less than the maximum possible weight, and it's balanced.

    willItFly(Arrays.asList(3), 5) -> true
    # 3 is less than the maximum possible weight, and it's balanced.
     */
    public boolean willItFly(List<Integer> q, int w) {
        if (q.stream().reduce(0, Integer::sum) > w) {
            return false;
        }
        int i = 0, j = q.size() - 1;
        while (i < j) {
            if (!Objects.equals(q.get(i), q.get(j))) {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
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
    Write a function that returns True if the object q will fly, and False otherwise.
    The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.

    Example:
    willItFly(Arrays.asList(1, 2), 5) -> false
    # 1+2 is less than the maximum possible weight, but it's unbalanced.

    willItFly(Arrays.asList(3, 2, 3), 1) -> false
    # it's balanced, but 3+2+3 is more than the maximum possible weight.

    willItFly(Arrays.asList(3, 2, 3), 9) -> true
    # 3+2+3 is less than the maximum possible weight, and it's balanced.

    willItFly(Arrays.asList(3), 5) -> true
    # 3 is less than the maximum possible weight, and it's balanced.
     */
    public boolean willItFly(List<Integer> q, int w) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
