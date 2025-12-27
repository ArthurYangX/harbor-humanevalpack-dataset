# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
    >>> intersperse(List.of(), 4)
    []
    >>> intersperse(Arrays.asList(1, 2, 3), 4)
    [1, 4, 2, 4, 3]
     */
    public List<Integer> intersperse(List<Integer> numbers, int delimiter) {
        if (numbers.size() == 0) {
            return List.of();
        }
        List<Integer> result = new ArrayList<>(List.of());
        for (int i = 0; i < numbers.size() - 1; i++) {
            result.add(numbers.get(i));
            result.add(delimiter);
        }

        return result;
    }
}
```

# Instruction

Fix bugs in intersperse.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
    >>> intersperse(List.of(), 4)
    []
    >>> intersperse(Arrays.asList(1, 2, 3), 4)
    [1, 4, 2, 4, 3]
     */
    public List<Integer> intersperse(List<Integer> numbers, int delimiter) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
