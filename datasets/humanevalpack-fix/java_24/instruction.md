# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    For a given number n, find the largest number that divides n evenly, smaller than n
    >>> largestDivisor(15)
    5
     */
    public int largestDivisor(int n) {
        for (int i = n - 1; i > 0; i--) {
            if (n - i == 0) {
                return i;
            }
        }
        return 1;
    }
}
```

# Instruction

Fix bugs in largestDivisor.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    For a given number n, find the largest number that divides n evenly, smaller than n
    >>> largestDivisor(15)
    5
     */
    public int largestDivisor(int n) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
