# Context

You are given a reference implementation (canonical solution) to explain.

```go
// brackets is a string of "<" and ">".
// return true if every opening bracket has a corresponding closing bracket.
// 
// >>> CorrectBracketing("<")
// false
// >>> CorrectBracketing("<>")
// true
// >>> CorrectBracketing("<<><>>")
// true
// >>> CorrectBracketing("><<>")
// false
func CorrectBracketing(brackets string) bool {
    l := len(brackets)
	count := 0
	for index := 0; index < l; index++ {
		if brackets[index] == '<' {
			count++
		} else if brackets[index] == '>' {
			count--
		}
		if count < 0 {
			return false
		}
	}
    if count == 0 {
        return true
    } else {
        return false
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


// brackets is a string of "<" and ">".
// return true if every opening bracket has a corresponding closing bracket.
// 
// >>> CorrectBracketing("<")
// false
// >>> CorrectBracketing("<>")
// true
// >>> CorrectBracketing("<<><>>")
// true
// >>> CorrectBracketing("><<>")
// false
func CorrectBracketing(brackets string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
