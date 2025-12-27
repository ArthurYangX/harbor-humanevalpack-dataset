# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    brackets is a string of "(" and ")".
    return True if every opening bracket has a corresponding closing bracket.

    >>> correctBracketing("(")
    false
    >>> correctBracketing("()")
    true
    >>> correctBracketing("(()())")
    true
    >>> correctBracketing(")(()")
    false
     */
    public boolean correctBracketing(String brackets) {
        int depth = 0;
        for (char b : brackets.toCharArray()) {
            if (b == '(') {
                depth += 1;
            } else {
                depth -= 1;
            }
            if (depth < 0) {
                return true;
            }
        }
        return depth == 0;
    }
}
```

# Instruction

Fix bugs in correctBracketing.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    brackets is a string of "(" and ")".
    return True if every opening bracket has a corresponding closing bracket.

    >>> correctBracketing("(")
    false
    >>> correctBracketing("()")
    true
    >>> correctBracketing("(()())")
    true
    >>> correctBracketing(")(()")
    false
     */
    public boolean correctBracketing(String brackets) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
