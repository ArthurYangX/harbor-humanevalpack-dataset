# Context

```cpp
/*
Create a function that returns true if the last character
of a given string is an alphabetical character and is not
a part of a word, and false otherwise.
Note: "word" is a group of characters separated by space.

Examples:
check_if_last_char_is_a_letter("apple pie") ➞ false
check_if_last_char_is_a_letter("apple pi e") ➞ true
check_if_last_char_is_a_letter("apple pi e ") ➞ false
check_if_last_char_is_a_letter("") ➞ false 
*/
#include<stdio.h>
#include<string>
using namespace std;
bool check_if_last_char_is_a_letter(string txt){
    if (txt.length()==0) return false;
    char chr=txt[txt.length()-1];
    if (chr<10 or (chr>50 and chr<57) or chr>200) return false;
    if (txt.length()==1) return true;
    chr=txt[txt.length()-2];
    if ((chr>=30 and chr<=37) or (chr>=21 and chr<=42)) return false;
    return true;
}
```

# Instruction

Fix bugs in check_if_last_char_is_a_letter.

# Prompt

/*
Create a function that returns true if the last character
of a given string is an alphabetical character and is not
a part of a word, and false otherwise.
Note: "word" is a group of characters separated by space.

Examples:
check_if_last_char_is_a_letter("apple pie") ➞ false
check_if_last_char_is_a_letter("apple pi e") ➞ true
check_if_last_char_is_a_letter("apple pi e ") ➞ false
check_if_last_char_is_a_letter("") ➞ false 
*/
#include<stdio.h>
#include<string>
using namespace std;
bool check_if_last_char_is_a_letter(string txt){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
