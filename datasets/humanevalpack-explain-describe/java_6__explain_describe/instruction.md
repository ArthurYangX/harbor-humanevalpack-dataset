# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
    For each of the group, output the deepest level of nesting of parentheses.
    E.g. (()()) has maximum two levels of nesting while ((())) has three.

    >>> parseNestedParens("(()()) ((())) () ((())()())")
    [2, 3, 1, 3]
     */
    public List<Integer> parseNestedParens(String paren_string) {
        String[] groups = paren_string.split(" ");
        List<Integer> result = new ArrayList<>(List.of());
        for (String group : groups) {
            if (group.length() > 0) {
                int depth = 0;
                int max_depth = 0;
                for (char c : group.toCharArray()) {
                    if (c == '(') {
                        depth += 1;
                        max_depth = Math.max(depth, max_depth);
                    } else {
                        depth -= 1;
                    }
                }
                result.add(max_depth);
            }
        }
        return result;
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
    Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
    For each of the group, output the deepest level of nesting of parentheses.
    E.g. (()()) has maximum two levels of nesting while ((())) has three.

    >>> parseNestedParens("(()()) ((())) () ((())()())")
    [2, 3, 1, 3]
     */
    public List<Integer> parseNestedParens(String paren_string) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
