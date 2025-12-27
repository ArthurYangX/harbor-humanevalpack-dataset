# Context

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
    For example:
    words_string("Hi, my name is John").equals(Arrays.asList("Hi", "my", "name", "is", "John"]
    words_string("One, two, three, four, five, six").equals(Arrays.asList("One", "two", "three", "four", "five", "six"]
     */
    public List<String> wordStrings(String s) {
        if (s.length() == 0) {
            return List.of();
        }
        StringBuilder sb = new StringBuilder();

        for (char letter : s.toCharArray()) {
            if (letter == ',') {
                sb.append(',');
            } else {
                sb.append(letter);
            }
        }

        return new ArrayList<>(Arrays.asList(sb.toString().split("\s+" )));
    }
}
```

# Instruction

Fix bugs in words_string.

# Prompt

import java.util.*;
import java.lang.*;

class Solution {
    /**
    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
    For example:
    words_string("Hi, my name is John").equals(Arrays.asList("Hi", "my", "name", "is", "John"]
    words_string("One, two, three, four, five, six").equals(Arrays.asList("One", "two", "three", "four", "five", "six"]
     */
    public List<String> wordStrings(String s) {


# Instructions

Implement your solution in `solution/solution.java`.
Ensure your submission is self-contained and compiles/runs correctly.

```
