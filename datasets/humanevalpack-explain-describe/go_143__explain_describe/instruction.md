# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// You are given a string representing a sentence,
// the sentence contains some words separated by a space,
// and you have to return a string that contains the words from the original sentence,
// whose lengths are prime numbers,
// the order of the words in the new string should be the same as the original one.
// 
// Example 1:
// Input: sentence = "This is a test"
// Output: "is"
// 
// Example 2:
// Input: sentence = "lets go for swimming"
// Output: "go for"
// 
// Constraints:
// * 1 <= len(sentence) <= 100
// * sentence contains only letters
func WordsInSentence(sentence string) string {
    new_lst := make([]string, 0)
    for _, word := range strings.Fields(sentence) {
        flg := 0
        if len(word) == 1 {
            flg = 1
        }
        for i := 2;i < len(word);i++ {
            if len(word)%i == 0 {
                flg = 1
            }
        }
        if flg == 0 || len(word) == 2 {
            new_lst = append(new_lst, word)
        }
    }
    return strings.Join(new_lst, " ")
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

import (
    "strings"
)

// You are given a string representing a sentence,
// the sentence contains some words separated by a space,
// and you have to return a string that contains the words from the original sentence,
// whose lengths are prime numbers,
// the order of the words in the new string should be the same as the original one.
// 
// Example 1:
// Input: sentence = "This is a test"
// Output: "is"
// 
// Example 2:
// Input: sentence = "lets go for swimming"
// Output: "go for"
// 
// Constraints:
// * 1 <= len(sentence) <= 100
// * sentence contains only letters
func WordsInSentence(sentence string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
