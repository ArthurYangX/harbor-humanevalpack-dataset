# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "sort"
)

// Write a function that accepts a list of strings as a parameter,
// deletes the strings that have odd lengths from it,
// and returns the resulted list with a sorted order,
// The list is always a list of strings and never an array of numbers,
// and it may contain duplicates.
// The order of the list should be ascending by length of each word, and you
// should return the list sorted by that rule.
// If two words have the same length, sort the list alphabetically.
// The function should return a list of strings in sorted order.
// You may assume that all words will have the same length.
// For example:
// assert list_sort(["aa", "a", "aaa"]) => ["aa"]
// assert list_sort(["ab", "a", "aaa", "cd"]) => ["ab", "cd"]
func SortedListSum(lst []string) []string {
    sort.SliceStable(lst, func(i, j int) bool {
        return lst[i] < lst[j]
    })
    new_lst := make([]string, 0)
    for _, i := range lst {
        if len(i)&1==0 {
            new_lst = append(new_lst, i)
        }
    }
    sort.SliceStable(new_lst, func(i, j int) bool {
        return len(new_lst[i]) < len(new_lst[j])
    })
    return new_lst
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
    "sort"
)

// Write a function that accepts a list of strings as a parameter,
// deletes the strings that have odd lengths from it,
// and returns the resulted list with a sorted order,
// The list is always a list of strings and never an array of numbers,
// and it may contain duplicates.
// The order of the list should be ascending by length of each word, and you
// should return the list sorted by that rule.
// If two words have the same length, sort the list alphabetically.
// The function should return a list of strings in sorted order.
// You may assume that all words will have the same length.
// For example:
// assert list_sort(["aa", "a", "aaa"]) => ["aa"]
// assert list_sort(["ab", "a", "aaa", "cd"]) => ["ab", "cd"]
func SortedListSum(lst []string) []string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
