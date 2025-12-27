# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a non-empty list of integers lst. add the even elements that are at odd indices..

    Examples:
        add(Arrays.asList(4, 2, 6, 7)) ==> 2
     */
    public int add(List<Integer> lst) {
        int sum = 0;
        for (int i = 1; i < lst.size(); i += 1) {
            if (lst.get(i) % 2 == 0) {
                sum += lst.get(i);
            }
        }
        return sum;
    }
}
```

# Instruction

Fix bugs in add.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a non-empty list of integers lst. add the even elements that are at odd indices..

    Examples:
        add(Arrays.asList(4, 2, 6, 7)) ==> 2
     */
    public int add(List<Integer> lst) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
