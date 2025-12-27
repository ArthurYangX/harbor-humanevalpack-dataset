# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// Given a dictionary, return true if all keys are strings in lower
// case or all keys are strings in upper case, else return false.
// The function should return false is the given dictionary is empty.
// Examples:
// CheckDictCase({"a":"apple", "b":"banana"}) should return true.
// CheckDictCase({"a":"apple", "A":"banana", "B":"banana"}) should return false.
// CheckDictCase({"a":"apple", 8:"banana", "a":"apple"}) should return false.
// CheckDictCase({"Name":"John", "Age":"36", "City":"Houston"}) should return false.
// CheckDictCase({"STATE":"NC", "ZIP":"12345" }) should return true.
func CheckDictCase(dict map[interface{}]interface{}) bool {
    if len(dict) == 0 {
        return false
    }
    state := "start"
    key := ""
    ok := false
    for k := range dict {
        if key, ok = k.(string); !ok {
            state = "mixed"
            break
        }
        if state == "start" {
            if key == strings.ToUpper(key) {
                state = "upper"
            } else if key == strings.ToLower(key) {
                state = "lower"
            } else {
                break
            }
        } else if (state == "upper" && key != strings.ToUpper(key)) || (state == "lower" && key != strings.ToLower(key)) {
            state = "mixed"
            break
        } else {
            break
        }
    }
    return state == "upper" || state == "lower"
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

import (
    "strings"
)

// Given a dictionary, return true if all keys are strings in lower
// case or all keys are strings in upper case, else return false.
// The function should return false is the given dictionary is empty.
// Examples:
// CheckDictCase({"a":"apple", "b":"banana"}) should return true.
// CheckDictCase({"a":"apple", "A":"banana", "B":"banana"}) should return false.
// CheckDictCase({"a":"apple", 8:"banana", "a":"apple"}) should return false.
// CheckDictCase({"Name":"John", "Age":"36", "City":"Houston"}) should return false.
// CheckDictCase({"STATE":"NC", "ZIP":"12345" }) should return true.
func CheckDictCase(dict map[interface{}]interface{}) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
