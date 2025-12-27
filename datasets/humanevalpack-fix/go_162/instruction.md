# Context

```go
import (
    "crypto/md5"
    "fmt"
)

// Given a string 'text', return its md5 hash equivalent string.
// If 'text' is an empty string, return nil.
// 
// >>> StringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
func StringToMd5(text string) interface{} {
    if len(text) == 0 {
        return fmt.Sprintf("%x", md5.Sum([]byte("")))
    }
    return fmt.Sprintf("%x", md5.Sum([]byte(text)))
}
```

# Instruction

Fix bugs in StringToMd5.

# Prompt

import (
    "crypto/md5"
    "fmt"
)

// Given a string 'text', return its md5 hash equivalent string.
// If 'text' is an empty string, return nil.
// 
// >>> StringToMd5('Hello world') == '3e25960a79dbc69b674cd4ec67a72c62'
func StringToMd5(text string) interface{} {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
