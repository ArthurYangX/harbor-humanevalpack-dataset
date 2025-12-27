# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given a positive integer, obtain its roman numeral equivalent as a string,
  and return it in lowercase.
  Restrictions: 1 <= num <= 1000

  Examples:
  >>> intToMiniRoman(19) == 'xix'
  >>> intToMiniRoman(152) == 'clii'
  >>> intToMiniRoman(426) == 'cdxxvi'
  */
const intToMiniRoman = (number) => {
  let num = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000]
  let sym = ['i', 'iv', 'v', 'ix', 'x', 'xl', 'l', 'xc', 'c', 'cd', 'd', 'cm', 'm']
  let i = 12
  let res = ''
  while (number) {
    let div = (number - number % num[i]) / num[i]
    number = number % num[i]
    while (div) {
      res += sym[i]
      div -= 1
    }
    i -= 1
  }
  return res
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
  Given a positive integer, obtain its roman numeral equivalent as a string,
  and return it in lowercase.
  Restrictions: 1 <= num <= 1000

  Examples:
  >>> intToMiniRoman(19) == 'xix'
  >>> intToMiniRoman(152) == 'clii'
  >>> intToMiniRoman(426) == 'cdxxvi'
  */
const intToMiniRoman = (number) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
