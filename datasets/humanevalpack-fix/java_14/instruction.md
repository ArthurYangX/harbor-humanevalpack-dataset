# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return list of all prefixes from shortest to longest of the input string
    >>> allPrefixes("abc")
    ["a", "ab", "abc"]
     */
    public List<String> allPrefixes(String string) {
        List<String> result = new ArrayList<>();

        for (int i = 1; i <= string.length()-1; i++) {
            result.add(string.substring(0, i));
        }
        return result;
    }
}
```

# Instruction

Fix bugs in allPrefixes.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Return list of all prefixes from shortest to longest of the input string
    >>> allPrefixes("abc")
    ["a", "ab", "abc"]
     */
    public List<String> allPrefixes(String string) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
