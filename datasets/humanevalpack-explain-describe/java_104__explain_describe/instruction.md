# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a list of positive integers x. return a sorted list of all
    elements that hasn't any even digit.

    Note: Returned list should be sorted in increasing order.
    
    For example:
    >>> uniqueDigits(Arrays.asList(15, 33, 1422, 1))
    [1, 15, 33]
    >>> uniqueDigits(Arrays.asList(152, 323, 1422, 10))
    []
     */
    public List<Integer> uniqueDigits(List<Integer> x) {
        List<Integer> odd_digit_elements = new ArrayList<>();
        for (int i : x) {
            boolean is_unique = true;
            for (char c : String.valueOf(i).toCharArray()) {
                if ((c - '0') % 2 == 0) {
                    is_unique = false;
                    break;
                }
            }
            if (is_unique) {
                odd_digit_elements.add(i);
            }
        }
        Collections.sort(odd_digit_elements);
        return odd_digit_elements;
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
    Given a list of positive integers x. return a sorted list of all
    elements that hasn't any even digit.

    Note: Returned list should be sorted in increasing order.
    
    For example:
    >>> uniqueDigits(Arrays.asList(15, 33, 1422, 1))
    [1, 15, 33]
    >>> uniqueDigits(Arrays.asList(152, 323, 1422, 10))
    []
     */
    public List<Integer> uniqueDigits(List<Integer> x) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
