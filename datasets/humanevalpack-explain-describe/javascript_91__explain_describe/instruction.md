# Context

You are given a reference implementation (canonical solution) to explain.

```js
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
  if (S[0] == 'I' && S[1] == ' ') { t = 1 }
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

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
