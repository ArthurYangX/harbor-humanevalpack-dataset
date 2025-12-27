# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*You are given a word. Your task is to find the closest vowel that stands between 
  two consonants from the right side of the word (case sensitive).
  
  Vowels in the beginning and ending doesn't count. Return empty string if you didn't
  find any vowel met the above condition. 

  You may assume that the given string contains English letter only.

  Example:
  getClosestVowel("yogurt") ==> "u"
  getClosestVowel("FULL") ==> "U"
  getClosestVowel("quick") ==> ""
  getClosestVowel("ab") ==> ""
  */
const getClosestVowel = (word) => {
  for (let i = word.length - 2; i > 0; i--) {
    if (
      !(word[i] != 'a' && word[i] != 'e' && word[i] != 'i' && word[i] != 'o' && word[i] != 'u' && word[i] != 'A' &&
        word[i] != 'U' && word[i] != 'O' && word[i] != 'I' && word[i] != 'E')
      &&
      (word[i + 1] != 'a' && word[i + 1] != 'e' && word[i + 1] != 'i' && word[i + 1] != 'o' && word[i + 1] != 'u' && word[i + 1] != 'A' &&
        word[i + 1] != 'U' && word[i + 1] != 'O' && word[i + 1] != 'I' && word[i + 1] != 'E')
      &&
      (word[i - 1] != 'a' && word[i - 1] != 'e' && word[i - 1] != 'i' && word[i - 1] != 'o' && word[i - 1] != 'u' && word[i - 1] != 'A' &&
        word[i - 1] != 'U' && word[i - 1] != 'O' && word[i - 1] != 'I' && word[i - 1] != 'E')
    ) {
      return word[i]
    }
  }
  return ''
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

/*You are given a word. Your task is to find the closest vowel that stands between 
  two consonants from the right side of the word (case sensitive).
  
  Vowels in the beginning and ending doesn't count. Return empty string if you didn't
  find any vowel met the above condition. 

  You may assume that the given string contains English letter only.

  Example:
  getClosestVowel("yogurt") ==> "u"
  getClosestVowel("FULL") ==> "U"
  getClosestVowel("quick") ==> ""
  getClosestVowel("ab") ==> ""
  */
const getClosestVowel = (word) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
