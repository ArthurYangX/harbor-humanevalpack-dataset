# Context

```javascript
/*Circular shift the digits of the integer x, shift the digits right by shift
  and return the result as a string.
  If shift > number of digits, return digits reversed.
  >>> circularShift(12, 1)
  "21"
  >>> circularShift(12, 2)
  "12"
  */
const circularShift = (x, shift) => {
  s = x.toString();
  if (shift > s.length)
    return s.split('').reverse().join('');
  else
    return s.slice(0, -shift) + s.slice(-shift);
}
```

# Instruction

Fix bugs in circularShift.

# Prompt

/*Circular shift the digits of the integer x, shift the digits right by shift
  and return the result as a string.
  If shift > number of digits, return digits reversed.
  >>> circularShift(12, 1)
  "21"
  >>> circularShift(12, 2)
  "12"
  */
const circularShift = (x, shift) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
