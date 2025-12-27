# Context

```javascript
/*
  Checks if given string is a palindrome
  >>> isPalindrome('')
  true
  >>> isPalindrome('aba')
  true
  >>> isPalindrome('aaaaa')
  true
  >>> isPalindrome('zbcd')
  false
  */
const isPalindrome = (text) => {
  for (let i = 0; i < text.length; i++)
    if (text[i] != text.at(-i))
      return false;
  return true;
}
```

# Instruction

Fix bugs in isPalindrome.

# Prompt

/*
  Checks if given string is a palindrome
  >>> isPalindrome('')
  true
  >>> isPalindrome('aba')
  true
  >>> isPalindrome('aaaaa')
  true
  >>> isPalindrome('zbcd')
  false
  */
const isPalindrome = (text) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
