# Context

```javascript
/* brackets is a string of "(" and ")".
  return true if every opening bracket has a corresponding closing bracket.

  >>> correctBracketing("(")
  false
  >>> correctBracketing("()")
  true
  >>> correctBracketing("(()())")
  true
  >>> correctBracketing(")(()")
  false
  */
const correctBracketing = (brackets) => {
  var depth = 0;
  for (const b of brackets) {
    if (b == "(")
      depth += 1;
    else
      depth -= 1;
    if (depth < 0)
      return true;
  }
  return depth == 0;
}
```

# Instruction

Fix bugs in correctBracketing.

# Prompt

/* brackets is a string of "(" and ")".
  return true if every opening bracket has a corresponding closing bracket.

  >>> correctBracketing("(")
  false
  >>> correctBracketing("()")
  true
  >>> correctBracketing("(()())")
  true
  >>> correctBracketing(")(()")
  false
  */
const correctBracketing = (brackets) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
