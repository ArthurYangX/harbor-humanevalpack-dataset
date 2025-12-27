# Context

```javascript
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
    if (p > 122) { p -= 24 }
    t += String.fromCharCode(p)
  }
  return t
}
```

# Instruction

Fix bugs in encrypt.

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

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
