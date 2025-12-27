# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given a map, return True if all keys are strings in lower
    case or all keys are strings in upper case, else return False.
    The function should return False is the given map is empty.
    Examples:
    checkDictCase({"a":"apple", "b":"banana"}) should return True.
    checkDictCase({"a":"apple", "A":"banana", "B":"banana"}) should return False.
    checkDictCase({"a":"apple", 8:"banana", "a":"apple"}) should return False.
    checkDictCase({"Name":"John", "Age":"36", "City":"Houston"}) should return False.
    checkDictCase({"STATE":"NC", "ZIP":"12345" }) should return True.
     */
    public boolean checkDictCase(Map<Object, Object> dict) {
        if (dict.isEmpty()) {
            return false;
        }
        String state = "start";
        for (Map.Entry entry : dict.entrySet()) {
            if (!(entry.getKey() instanceof String key)) {
                state = "mixed";
                break;
            }
            boolean is_upper = true, is_lower = true;
            for (char c : key.toCharArray()) {
                if (Character.isLowerCase(c)) {
                    is_upper = false;
                } else if (Character.isUpperCase(c)) {
                    is_lower = false;
                } else {
                    is_upper = false;
                    is_lower = false;
                }
            }
            if (state.equals("start")) {
                if (is_upper) {
                    state = "upper";
                } else if (is_lower) {
                    state = "lower";
                } else {
                    break;
                }
            } else if ((state.equals("upper") && !is_upper) || (state.equals("lower") && !is_lower)) {
                state = "mixed";
                break;
            }
        }
        return state.equals("upper") || state.equals("lower");
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
    Given a map, return True if all keys are strings in lower
    case or all keys are strings in upper case, else return False.
    The function should return False is the given map is empty.
    Examples:
    checkDictCase({"a":"apple", "b":"banana"}) should return True.
    checkDictCase({"a":"apple", "A":"banana", "B":"banana"}) should return False.
    checkDictCase({"a":"apple", 8:"banana", "a":"apple"}) should return False.
    checkDictCase({"Name":"John", "Age":"36", "City":"Houston"}) should return False.
    checkDictCase({"STATE":"NC", "ZIP":"12345" }) should return True.
     */
    public boolean checkDictCase(Map<Object, Object> dict) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
