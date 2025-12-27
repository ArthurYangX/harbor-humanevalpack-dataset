# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given an integer. return a tuple that has the number of even and odd digits respectively.
    
     Example:
        evenOddCount(-12) ==> (1, 1)
        evenOddCount(123) ==> (1, 2)
     */
    public List<Integer> evenOddCount(int num) {
        int even_count = 0, odd_count = 0;
        for (char i : String.valueOf(Math.abs(num)).toCharArray()) {
            if (i % 2 == 0) {
                even_count += 1;
            }
        }
        return Arrays.asList(even_count, odd_count);
    }
}
```

# Instruction

Fix bugs in evenOddCount.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given an integer. return a tuple that has the number of even and odd digits respectively.
    
     Example:
        evenOddCount(-12) ==> (1, 1)
        evenOddCount(123) ==> (1, 2)
     */
    public List<Integer> evenOddCount(int num) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
