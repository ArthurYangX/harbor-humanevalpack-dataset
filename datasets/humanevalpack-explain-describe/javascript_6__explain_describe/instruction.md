# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
  For each of the group, output the deepest level of nesting of parentheses.
  E.g. (()()) has maximum two levels of nesting while ((())) has three.

  >>> parseNestedParens('(()()) ((())) () ((())()())')
  [2, 3, 1, 3]
  */
const parseNestedParens = (paren_string) => {
  var parseParenGroup = function (s) {
    let depth = 0, max_depth = 0;
    for (const c of s) {
      if (c == '(') {
        depth += 1;
        max_depth = Math.max(max_depth, depth);
      } else {
        depth -= 1;
      }
    }
    return max_depth;
  }
  return paren_string.split(' ')
          .filter(x => x != '')
          .map(x => parseParenGroup(x));
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

/* Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
  For each of the group, output the deepest level of nesting of parentheses.
  E.g. (()()) has maximum two levels of nesting while ((())) has three.

  >>> parseNestedParens('(()()) ((())) () ((())()())')
  [2, 3, 1, 3]
  */
const parseNestedParens = (paren_string) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
