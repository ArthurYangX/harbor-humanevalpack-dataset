# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Find how many times a given substring can be found in the original string. Count overlaping cases.
    >>> howManyTimes("", "a")
    0
    >>> howManyTimes("aaa", "a")
    3
    >>> howManyTimes("aaaa", "aa")
    3
     */
    public int howManyTimes(String string, String substring) {
        int times = 0;

        for (int i = 0; i < string.length() - substring.length(); i++) {
            if (string.substring(i, i + substring.length()).equals(substring)) {
                times += 1;
            }
        }

        return times;
    }
}
```

# Instruction

Fix bugs in howManyTimes.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Find how many times a given substring can be found in the original string. Count overlaping cases.
    >>> howManyTimes("", "a")
    0
    >>> howManyTimes("aaa", "a")
    3
    >>> howManyTimes("aaaa", "aa")
    3
     */
    public int howManyTimes(String string, String substring) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
