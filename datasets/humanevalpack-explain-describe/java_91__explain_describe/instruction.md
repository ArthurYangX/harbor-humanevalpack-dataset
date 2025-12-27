# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    You'll be given a string of words, and your task is to count the number
    of boredoms. A boredom is a sentence that starts with the word "I".
    Sentences are delimited by '.', '?' or '!'.

    For example:
    >>> isBored("Hello world")
    0
    >>> isBored("The sky is blue. The sun is shining. I love this weather")
    1
     */
    public int isBored(String S) {
        String [] sentences = S.split("[.?!]\s*");
        int count = 0;
        for (String sentence : sentences) {
            if (sentence.subSequence(0, 2).equals("I ")) {
                count += 1;
            }
        }
        return count;
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
    You'll be given a string of words, and your task is to count the number
    of boredoms. A boredom is a sentence that starts with the word "I".
    Sentences are delimited by '.', '?' or '!'.

    For example:
    >>> isBored("Hello world")
    0
    >>> isBored("The sky is blue. The sun is shining. I love this weather")
    1
     */
    public int isBored(String S) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
