# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Out of list of strings, return the longest one. Return the first one in case of multiple
  strings of the same length. Return null in case the input list is empty.
  >>> longest([])

  >>> longest(['a', 'b', 'c'])
  'a'
  >>> longest(['a', 'bb', 'ccc'])
  'ccc'
  */
const longest = (strings) => {
  if (!Array.isArray(strings) || strings.length == 0)
    return null;
  var maxlen = Math.max(...strings.map(x => x.length));
  for (const s of strings) {
    if (s.length == maxlen) {
      return s;
    }
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

/* Out of list of strings, return the longest one. Return the first one in case of multiple
  strings of the same length. Return null in case the input list is empty.
  >>> longest([])

  >>> longest(['a', 'b', 'c'])
  'a'
  >>> longest(['a', 'bb', 'ccc'])
  'ccc'
  */
const longest = (strings) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
