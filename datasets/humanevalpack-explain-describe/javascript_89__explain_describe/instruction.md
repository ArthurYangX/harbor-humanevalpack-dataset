# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Create a function encrypt that takes a string as an argument and
  returns a string encrypted with the alphabet being rotated. 
  The alphabet should be rotated in a manner such that the letters 
  shift down by two multiplied to two places.
  For example:
  encrypt('hi') returns 'lm'
  encrypt('asdfghjkl') returns 'ewhjklnop'
  encrypt('gf') returns 'kj'
  encrypt('et') returns 'ix'
  */
const encrypt = (s) => {
  let t = ''
  for (let i = 0; i < s.length; i++) {
    let p = s[i].charCodeAt() + 4
    if (p > 122) { p -= 26 }
    t += String.fromCharCode(p)
  }
  return t
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

/*Create a function encrypt that takes a string as an argument and
  returns a string encrypted with the alphabet being rotated. 
  The alphabet should be rotated in a manner such that the letters 
  shift down by two multiplied to two places.
  For example:
  encrypt('hi') returns 'lm'
  encrypt('asdfghjkl') returns 'ewhjklnop'
  encrypt('gf') returns 'kj'
  encrypt('et') returns 'ix'
  */
const encrypt = (s) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
