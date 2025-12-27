# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a list of numbers, return the sum of squares of the numbers
    in the list that are odd. Ignore numbers that are negative or not integers.

    doubleTheDifference(Arrays.asList(1, 3, 2, 0)) == 1 + 9 + 0 + 0 = 10
    doubleTheDifference(Arrays.asList(-1, -2, 0)) == 0
    doubleTheDifference(Arrays.asList(9, -2)) == 81
    doubleTheDifference(Arrays.asList(0)) == 0

    If the input list is empty, return 0.
     */
    public int doubleTheDifference(List<Object> lst) {
        return lst.stream().filter(i -> i instanceof Integer p && p > 0 && p % 2 != 0).map(i -> (Integer) i * (Integer) i).reduce(Integer::sum).orElse(0);
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
    Given a list of numbers, return the sum of squares of the numbers
    in the list that are odd. Ignore numbers that are negative or not integers.

    doubleTheDifference(Arrays.asList(1, 3, 2, 0)) == 1 + 9 + 0 + 0 = 10
    doubleTheDifference(Arrays.asList(-1, -2, 0)) == 0
    doubleTheDifference(Arrays.asList(9, -2)) == 81
    doubleTheDifference(Arrays.asList(0)) == 0

    If the input list is empty, return 0.
     */
    public int doubleTheDifference(List<Object> lst) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
