# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;
import java.util.stream.Collectors;

class Solution {
    /**
    Return list with elements incremented by 1.
    >>> incrList(Arrays.asList(1, 2, 3))
    [2, 3, 4]
    >>> incrList(Arrays.asList(5, 3, 5, 2, 3, 3, 9, 0, 123))
    [6, 4, 6, 3, 4, 4, 10, 1, 124]
     */
    public List<Integer> incrList(List<Integer> l) {
        return l.stream().map(p -> p + 1).collect(Collectors.toList());
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
import java.util.stream.Collectors;

class Solution {
    /**
    Return list with elements incremented by 1.
    >>> incrList(Arrays.asList(1, 2, 3))
    [2, 3, 4]
    >>> incrList(Arrays.asList(5, 3, 5, 2, 3, 3, 9, 0, 123))
    [6, 4, 6, 3, 4, 4, 10, 1, 124]
     */
    public List<Integer> incrList(List<Integer> l) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
