# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return maximum element in the list.
    >>> maxElement(Arrays.asList(1, 2, 3))
    3
    >>> maxElement(Arrays.asList(5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10))
    123
     */
    public int maxElement(List<Integer> l) {
        return Collections.max(l);
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
    Return maximum element in the list.
    >>> maxElement(Arrays.asList(1, 2, 3))
    3
    >>> maxElement(Arrays.asList(5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10))
    123
     */
    public int maxElement(List<Integer> l) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
