# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
    >>> flipCase("Hello")
    "hELLO"
     */
    public String flipCase(String string) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < string.length(); i++) {
            if (Character.isLowerCase(string.charAt(i))) {
                sb.append(Character.toUpperCase(string.charAt(i)));
            } else {
                sb.append(Character.toLowerCase(string.charAt(i)));
            }
        }
        return sb.toString();
    }
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
    >>> flipCase("Hello")
    "hELLO"
     */
    public String flipCase(String string) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
