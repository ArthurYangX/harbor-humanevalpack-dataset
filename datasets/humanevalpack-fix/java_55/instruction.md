# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return n-th Fibonacci number.
    >>> fib(10)
    55
    >>> fib(1)
    1
    >>> fib(8)
    21
     */
    public int fib(int n) {
        if (n == 0) {
            return 0;
        }
        if (n == 1) {
            return 1;
        }
        if (n == 2) {
            return 2;
        }
        return fib(n - 1) + fib(n - 2);
    }
}
```

# Instruction

Fix bugs in fib.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return n-th Fibonacci number.
    >>> fib(10)
    55
    >>> fib(1)
    1
    >>> fib(8)
    21
     */
    public int fib(int n) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
