# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Check if two words have the same characters.
  >>> sameChars('eabcdzzzz', 'dddzzzzzzzddeddabc')
  true
  >>> sameChars('abcd', 'dddddddabc')
  true
  >>> sameChars('dddddddabc', 'abcd')
  true
  >>> sameChars('eabcd', 'dddddddabc')
  false
  >>> sameChars('abcd', 'dddddddabce')
  false
  >>> sameChars('eabcdzzzz', 'dddzzzzzzzddddabc')
  false
  */
const sameChars = (s0, s1) => {
  return JSON.stringify([...new Set(s0)].sort()) === JSON.stringify([...new Set(s1)].sort());
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

/*
  Check if two words have the same characters.
  >>> sameChars('eabcdzzzz', 'dddzzzzzzzddeddabc')
  true
  >>> sameChars('abcd', 'dddddddabc')
  true
  >>> sameChars('dddddddabc', 'abcd')
  true
  >>> sameChars('eabcd', 'dddddddabc')
  false
  >>> sameChars('abcd', 'dddddddabce')
  false
  >>> sameChars('eabcdzzzz', 'dddzzzzzzzddddabc')
  false
  */
const sameChars = (s0, s1) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
