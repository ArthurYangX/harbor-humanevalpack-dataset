# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Filter given list of any values only for integers
    >>> filter_integers(Arrays.asList('a', 3.14, 5))
    [5]
    >>> filter_integers(Arrays.asList(1, 2, 3, "abc", Map.of(), List.of()))
    [1, 2, 3]
     */
    public List<Integer> filterIntegers(List<Object> values) {
        List<Integer> result = new ArrayList<>();
        for (Object x : values) {
            if (x instanceof Integer) {
                result.add((Integer) x);
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
    Filter given list of any values only for integers
    >>> filter_integers(Arrays.asList('a', 3.14, 5))
    [5]
    >>> filter_integers(Arrays.asList(1, 2, 3, "abc", Map.of(), List.of()))
    [1, 2, 3]
     */
    public List<Integer> filterIntegers(List<Object> values) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
