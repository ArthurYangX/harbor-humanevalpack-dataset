# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
    should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
    alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
    Examples
    splitWords("Hello world!") == ["Hello", "world!"]
    splitWords("Hello,world!") == ["Hello", "world!"]
    splitWords("abcdef") == 3
     */
    public Object splitWords(String txt) {
        if (txt.contains(" " )) {
            return Arrays.asList(txt.split(" " ));
        } else if (txt.contains("," )) {
            return Arrays.asList(txt.split("[,\s]" ));
        } else {
            int count = 0;
            for (char c : txt.toCharArray()) {
                if (Character.isLowerCase(c) && (c - 'a') % 2 == 1) {
                    count += 1;
                }
            }
            return count;
        }
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
    Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
    should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
    alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
    Examples
    splitWords("Hello world!") == ["Hello", "world!"]
    splitWords("Hello,world!") == ["Hello", "world!"]
    splitWords("abcdef") == 3
     */
    public Object splitWords(String txt) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
