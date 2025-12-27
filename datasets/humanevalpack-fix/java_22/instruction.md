# Context

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
                values.add((Integer) x);
            }
        }
        return result;
    }
}
```

# Instruction

Fix bugs in filterIntegers.

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

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
