# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string, find out how many distinct characters (regardless of case) does it consist of
    >>> countDistinctCharacters("xyzXYZ")
    3
    >>> countDistinctCharacters("Jerry")
    4
     */
    public int countDistinctCharacters(String string) {
        Set<Character> set = new HashSet<>();
        for (char c : string.toCharArray()) {
            set.add(c);
        }
        return set.size();
    }
}
```

# Instruction

Fix bugs in countDistinctCharacters.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string, find out how many distinct characters (regardless of case) does it consist of
    >>> countDistinctCharacters("xyzXYZ")
    3
    >>> countDistinctCharacters("Jerry")
    4
     */
    public int countDistinctCharacters(String string) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
