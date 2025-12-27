# Context

```javascript
/*
  You'll be given a string of words, and your task is to count the number
  of boredoms. A boredom is a sentence that starts with the word "I".
  Sentences are delimited by '.', '?' or '!'.
 
  For example:
  >>> isBored("Hello world")
  0
  >>> isBored("The sky is blue. The sun is shining. I love this weather")
  1
  */
const isBored = (S) => {
  let t = 0
  if (S[0] == ' ' && S[1] == 'I') { t = 1 }
  for (let i = 0; i < S.length; i++) {
    if (S[i] == '.' || S[i] == '!' || S[i] == '?') {
      if (S[i + 1] == ' ' && S[i + 2] == 'I' && S[i + 3] == ' ') {
        t++
      }
    }
  }
  return t
}
```

# Instruction

Fix bugs in isBored.

# Prompt

/*
  You'll be given a string of words, and your task is to count the number
  of boredoms. A boredom is a sentence that starts with the word "I".
  Sentences are delimited by '.', '?' or '!'.
 
  For example:
  >>> isBored("Hello world")
  0
  >>> isBored("The sky is blue. The sun is shining. I love this weather")
  1
  */
const isBored = (S) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
