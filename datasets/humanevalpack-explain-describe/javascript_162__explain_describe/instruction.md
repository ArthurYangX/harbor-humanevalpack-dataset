# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given a string 'text', return its md5 hash equivalent string.
  If 'text' is an empty string, return null.

  >>> stringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
  */
const stringToMd5 = (text) => {
  if (text == '') { return null }
  var md5 = require('js-md5')
  return md5(text)
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
  Given a string 'text', return its md5 hash equivalent string.
  If 'text' is an empty string, return null.

  >>> stringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
  */
const stringToMd5 = (text) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
