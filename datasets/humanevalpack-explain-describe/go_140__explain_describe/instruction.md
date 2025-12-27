# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Given a string text, replace all spaces in it with underscores,
// and if a string has more than 2 consecutive spaces,
// then replace all consecutive spaces with -
// 
// FixSpaces("Example") == "Example"
// FixSpaces("Example 1") == "Example_1"
// FixSpaces(" Example 2") == "_Example_2"
// FixSpaces(" Example   3") == "_Example-3"
func FixSpaces(text string) string {
    new_text := make([]byte, 0)
    i := 0
    start, end := 0, 0
    for i < len(text) {
        if text[i] == ' ' {
            end++
        } else {
            switch {
            case end - start > 2:
                new_text = append(new_text, '-')
            case end - start > 0:
                for n := 0;n < end-start;n++ {
                    new_text = append(new_text, '_')
                }
            }
            new_text = append(new_text, text[i])
            start, end = i+1, i+1
        }
        i+=1
    }
    if end - start > 2 {
        new_text = append(new_text, '-')
    } else if end - start > 0 {
        new_text = append(new_text, '_')
    }
    return string(new_text)
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


// Given a string text, replace all spaces in it with underscores,
// and if a string has more than 2 consecutive spaces,
// then replace all consecutive spaces with -
// 
// FixSpaces("Example") == "Example"
// FixSpaces("Example 1") == "Example_1"
// FixSpaces(" Example 2") == "_Example_2"
// FixSpaces(" Example   3") == "_Example-3"
func FixSpaces(text string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
