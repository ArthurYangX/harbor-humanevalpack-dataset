# Context

```cpp
/*
Checks if given string is a palindrome
>>> is_palindrome("")
true
>>> is_palindrome("aba")
true
>>> is_palindrome("aaaaa")
true
>>> is_palindrome("zbcd")
false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool is_palindrome(string text){
    string pr(text.rend(),text.rbegin());
    return pr==text;
}
```

# Instruction

Fix bugs in is_palindrome.

# Prompt

/*
Checks if given string is a palindrome
>>> is_palindrome("")
true
>>> is_palindrome("aba")
true
>>> is_palindrome("aaaaa")
true
>>> is_palindrome("zbcd")
false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool is_palindrome(string text){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
