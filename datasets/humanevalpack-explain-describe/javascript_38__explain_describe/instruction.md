# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  returns encoded string by cycling groups of three characters.
  */
const encodeCyclic = (s) => {
  var groups = [], groups2 = [];
  for (let i = 0; i < Math.floor((s.length + 2) / 3); i++) {
    groups.push(s.slice(3 * i, Math.min((3 * i + 3), s.length)));
  }
  for (const group of groups) {
    if (group.length == 3)
      groups2.push(group.slice(1) + group[0]);
    else
      groups2.push(group);
  }
  return groups2.join('');
}

/*
  takes as input string encoded with encode_cyclic function. Returns decoded string.
  */
const decodeCyclic = (s) => {
  return encodeCyclic(encodeCyclic(s));
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
  returns encoded string by cycling groups of three characters.
  */
const encodeCyclic = (s) => {
  var groups = [], groups2 = [];
  for (let i = 0; i < Math.floor((s.length + 2) / 3); i++) {
    groups.push(s.slice(3 * i, Math.min((3 * i + 3), s.length)));
  }
  for (const group of groups) {
    if (group.length == 3)
      groups2.push(group.slice(1) + group[0]);
    else
      groups2.push(group);
  }
  return groups2.join('');
}

/*
  takes as input string encoded with encode_cyclic function. Returns decoded string.
  */
const decodeCyclic = (s) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
