# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    The Brazilian factorial is defined as:
    brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
    where n > 0

    For example:
    >>> specialFactorial(4)
    288

    The function will receive an integer as input and should return the special
    factorial of this integer.
     */
    public long specialFactorial(int n) {
        long fact_i = 1, special_fact = 1;
        for (int i = 1; i <= n; i++) {
            i *= n;
            fact_i *= i;
            special_fact *= fact_i;
        }
        return special_fact;
    }
}
```

# Instruction

Fix bugs in specialFactorial.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    The Brazilian factorial is defined as:
    brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
    where n > 0

    For example:
    >>> specialFactorial(4)
    288

    The function will receive an integer as input and should return the special
    factorial of this integer.
     */
    public long specialFactorial(int n) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
