# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Concatenate list of strings into a single string
    >>> concatenate(List.of())
    ""
    >>> concatenate(Arrays.asList("a", "b", "c"))
    "abc"
     */
    public String concatenate(List<String> strings) {
        return String.join(" ", strings);
    }
}
```

# Instruction

Fix bugs in concatenate.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Concatenate list of strings into a single string
    >>> concatenate(List.of())
    ""
    >>> concatenate(Arrays.asList("a", "b", "c"))
    "abc"
     */
    public String concatenate(List<String> strings) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
