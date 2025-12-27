# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Check if two words have the same characters.
// >>> SameChars('eabcdzzzz', 'dddzzzzzzzddeddabc')
// true
// >>> SameChars('abcd', 'dddddddabc')
// true
// >>> SameChars('dddddddabc', 'abcd')
// true
// >>> SameChars('eabcd', 'dddddddabc')
// false
// >>> SameChars('abcd', 'dddddddabce')
// false
// >>> SameChars('eabcdzzzz', 'dddzzzzzzzddddabc')
// false
func SameChars(s0 string, s1 string) bool {
    set0 := make(map[int32]interface{})
	set1 := make(map[int32]interface{})
	for _, i := range s0 {
		set0[i] = nil
	}
	for _, i := range s1 {
		set1[i] = nil
	}
	for i, _ := range set0 {
		if _,ok:=set1[i];!ok{
			return false
		}
	}
	for i, _ := range set1 {
		if _,ok:=set0[i];!ok{
			return false
		}
	}
	return true
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


// Check if two words have the same characters.
// >>> SameChars('eabcdzzzz', 'dddzzzzzzzddeddabc')
// true
// >>> SameChars('abcd', 'dddddddabc')
// true
// >>> SameChars('dddddddabc', 'abcd')
// true
// >>> SameChars('eabcd', 'dddddddabc')
// false
// >>> SameChars('abcd', 'dddddddabce')
// false
// >>> SameChars('eabcdzzzz', 'dddzzzzzzzddddabc')
// false
func SameChars(s0 string, s1 string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
