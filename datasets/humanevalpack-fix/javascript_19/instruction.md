# Context

```javascript
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
          .join(' ');
}
```

# Instruction

Fix bugs in sortNumbers.

# Prompt

/* Input is a space-delimited string of numberals from 'zero' to 'nine'.
  Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
  Return the string with numbers sorted from smallest to largest
  >>> sortNumbers('three one five')
  'one three five'
  */
const sortNumbers = (numbers) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
