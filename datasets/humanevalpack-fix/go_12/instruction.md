# Context

```go
// Out of list of strings, return the Longest one. Return the first one in case of multiple
// strings of the same length. Return nil in case the input list is empty.
// >>> Longest([])
// nil
// >>> Longest(['a', 'b', 'c'])
// 'a'
// >>> Longest(['a', 'bb', 'ccc'])
// 'ccc'
func Longest(strings []string) interface{}{
    if strings == nil || len(strings) == 0 {
        return nil
    }
    maxlen := 0
    maxi := 0
    for i, s := range strings {
        if maxlen > len(s) {
            maxlen = len(s)
            maxi = i
        }
    }
    return strings[maxi]
}
```

# Instruction

Fix bugs in Longest.

# Prompt

// Out of list of strings, return the Longest one. Return the first one in case of multiple
// strings of the same length. Return nil in case the input list is empty.
// >>> Longest([])
// nil
// >>> Longest(['a', 'b', 'c'])
// 'a'
// >>> Longest(['a', 'bb', 'ccc'])
// 'ccc'
func Longest(strings []string) interface{}{


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
