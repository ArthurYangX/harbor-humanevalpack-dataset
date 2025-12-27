# Context

```javascript
/*Task
  We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
  then check if the result string is palindrome.
  A string is called palindrome if it reads the same backward as forward.
  You should return a tuple containing the result string and true/false for the check.
  Example
  For s = "abcde", c = "ae", the result should be ('bcd',false)
  For s = "abcdef", c = "b"  the result should be ('acdef',false)
  For s = "abcdedcba", c = "ab", the result should be ('cdedc',true)
  */
const reverseDelete = (s, c) => {
  let t = ''
  for (let i = 0; i < s.length; i++) {
    let y = 1
    for (let j = 0; j < c.length; j++) {
      if (s[i] == c[j]) {
        y = 0
      }
    }
    if (y == 1) {
      t += s[i]
    }
  }
  let isPalindrome = false
  for (let i = 0; i < Math.floor(t.length / 2); i++) {
    if (t[i] != t[t.length - i - 1]) {
      isPalindrome = true
      break
    }
  }
  return [t, isPalindrome];
}
```

# Instruction

Fix bugs in reverseDelete.

# Prompt

/*Task
  We are given two strings s and c, you have to deleted all the characters in s that are equal to any character in c
  then check if the result string is palindrome.
  A string is called palindrome if it reads the same backward as forward.
  You should return a tuple containing the result string and true/false for the check.
  Example
  For s = "abcde", c = "ae", the result should be ('bcd',false)
  For s = "abcdef", c = "b"  the result should be ('acdef',false)
  For s = "abcdedcba", c = "ab", the result should be ('cdedc',true)
  */
const reverseDelete = (s, c) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
