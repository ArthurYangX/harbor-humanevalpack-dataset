# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string text, replace all spaces in it with underscores,
    and if a string has more than 2 consecutive spaces,
    then replace all consecutive spaces with -

    fixSpaces("Example") == "Example"
    fixSpaces("Example 1") == "Example_1"
    fixSpaces(" Example 2") == "_Example_2"
    fixSpaces(" Example   3") == "_Example-3"
     */
    public String fixSpaces(String text) {
        StringBuilder sb = new StringBuilder();
        int start = 0, end = 0;
        for (int i = 0; i < text.length(); i++) {
            if (text.charAt(i) == ' ') {
                end += 1;
            } else {
                if (end - start > 2) {
                    sb.append('-');
                } else if (end - start > 0) {
                    sb.append("_".repeat(end - start));
                }
                sb.append(text.charAt(i));
                start = i + 1;
                end = i + 1;
            }
        }
        if (end - start > 2) {
            sb.append('-');
        } else if (end - start > 0) {
            sb.append("_".repeat(end - start));
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
    Given a string text, replace all spaces in it with underscores,
    and if a string has more than 2 consecutive spaces,
    then replace all consecutive spaces with -

    fixSpaces("Example") == "Example"
    fixSpaces("Example 1") == "Example_1"
    fixSpaces(" Example 2") == "_Example_2"
    fixSpaces(" Example   3") == "_Example-3"
     */
    public String fixSpaces(String text) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
