# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Input is a space-delimited string of numberals from 'zero' to 'nine'.
  Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
  Return the string with numbers sorted from smallest to largest
  >>> sortNumbers('three one five')
  'one three five'
  */
const sortNumbers = (numbers) => {
  const value_map = {
    'zero': 0,
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9
  };
  return numbers.split(' ')
          .filter(x => x != '')
          .sort((a, b) => value_map[a] - value_map[b])
          .join(' ');
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

/* Input is a space-delimited string of numberals from 'zero' to 'nine'.
  Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
  Return the string with numbers sorted from smallest to largest
  >>> sortNumbers('three one five')
  'one three five'
  */
const sortNumbers = (numbers) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
