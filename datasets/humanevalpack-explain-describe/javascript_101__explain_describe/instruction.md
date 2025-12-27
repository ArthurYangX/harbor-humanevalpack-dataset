# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  You will be given a string of words separated by commas or spaces. Your task is
  to split the string into words and return an array of the words.
  
  For example:
  wordsString("Hi, my name is John") == ["Hi", "my", "name", "is", "John"]
  wordsString("One, two, three, four, five, six") == ["One", "two", "three", "four", "five", "six"]
  */
const wordsString = (s) => {
  let t = ''
  let p = []
  let k = 0
  for (let i = 0; i < s.length; i++) {
    if (s[i] == ' ' || s[i] == ',') {
      if (k == 0) {
        k = 1;
        p.push(t);
        t = '';
      }
    }
    else {
      k = 0;
      t += s[i]
    }
  }
  if (t != '') {
    p.push(t);
  }
  return p
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
  You will be given a string of words separated by commas or spaces. Your task is
  to split the string into words and return an array of the words.
  
  For example:
  wordsString("Hi, my name is John") == ["Hi", "my", "name", "is", "John"]
  wordsString("One, two, three, four, five, six") == ["One", "two", "three", "four", "five", "six"]
  */
const wordsString = (s) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
