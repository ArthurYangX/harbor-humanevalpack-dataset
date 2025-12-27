# Context

```javascript
/*
  Given a string 'text', return its md5 hash equivalent string.
  If 'text' is an empty string, return null.

  >>> stringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
  */
const stringToMd5 = (text) => {
  if (text == '') { return null }
  var md5 = require('js-md5')
  return md5('text')
}
```

# Instruction

Fix bugs in stringToMd5.

# Prompt

/*
  Given a string 'text', return its md5 hash equivalent string.
  If 'text' is an empty string, return null.

  >>> stringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
  */
const stringToMd5 = (text) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
