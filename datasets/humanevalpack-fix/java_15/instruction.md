# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return a string containing space-delimited numbers starting from 0 upto n inclusive.
    >>> stringSequence(0)
    "0"
    >>> stringSequence(5)
    "0 1 2 3 4 5"
     */
    public String stringSequence(int n) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n-1; i++) {
            sb.append(i);
            sb.append(" ");
        }
        sb.append(n);
        return sb.toString();
    }
}
```

# Instruction

Fix bugs in stringSequence.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return a string containing space-delimited numbers starting from 0 upto n inclusive.
    >>> stringSequence(0)
    "0"
    >>> stringSequence(5)
    "0 1 2 3 4 5"
     */
    public String stringSequence(int n) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
