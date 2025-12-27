# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Task
  Write a function that takes a string as input and returns the sum of the upper characters only'
  ASCII codes.

  Examples:
      digitSum("") => 0
      digitSum("abAB") => 131
      digitSum("abcCd") => 67
      digitSum("helloE") => 69
      digitSum("woArBld") => 131
      digitSum("aAaaaXa") => 153
  */
const digitSum = (s) => {
  if (s == '') return 0;
  return s.split('').reduce((prev, char) => {
    let ord_char = char.charCodeAt(0)
    return prev + (ord_char > 64 && ord_char < 91 ? ord_char : 0);
  }, 0);
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

/*Task
  Write a function that takes a string as input and returns the sum of the upper characters only'
  ASCII codes.

  Examples:
      digitSum("") => 0
      digitSum("abAB") => 131
      digitSum("abcCd") => 67
      digitSum("helloE") => 69
      digitSum("woArBld") => 131
      digitSum("aAaaaXa") => 153
  */
const digitSum = (s) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
