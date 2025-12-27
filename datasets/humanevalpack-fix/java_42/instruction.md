# Context

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
        return l.stream().map(p -> p + 2).collect(Collectors.toList());
    }
}
```

# Instruction

Fix bugs in incrList.

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

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
