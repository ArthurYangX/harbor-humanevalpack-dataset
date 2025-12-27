# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.
    >>> changeBase(8, 3)
    "22"
    >>> changeBase(8, 2)
    "1000"
    >>> changeBase(7, 2)
    "111"
     */
    public String changeBase(int x, int base) {
        StringBuilder ret = new StringBuilder();
        while (x > 0) {
            ret.append(String.valueOf(x % base));
            x -= base;
        }
        return ret.reverse().toString();
    }
}
```

# Instruction

Fix bugs in changeBase.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.
    >>> changeBase(8, 3)
    "22"
    >>> changeBase(8, 2)
    "1000"
    >>> changeBase(7, 2)
    "111"
     */
    public String changeBase(int x, int base) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
