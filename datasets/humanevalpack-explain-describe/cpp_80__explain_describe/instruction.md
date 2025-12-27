# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You are given a string s.
Your task is to check if the string is happy or not.
A string is happy if its length is at least 3 and every 3 consecutive letters are distinct
For example:
is_happy("a") => false
is_happy("aa") => false
is_happy("abcd") => true
is_happy("aabb") => false
is_happy("adb") => true
is_happy("xyy") => false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool is_happy(string s){
    if (s.length()<3) return false;
    for (int i=2;i<s.length();i++)
    if (s[i]==s[i-1] or s[i]==s[i-2]) return false;
    return true;
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
You are given a string s.
Your task is to check if the string is happy or not.
A string is happy if its length is at least 3 and every 3 consecutive letters are distinct
For example:
is_happy("a") => false
is_happy("aa") => false
is_happy("abcd") => true
is_happy("aabb") => false
is_happy("adb") => true
is_happy("xyy") => false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool is_happy(string s){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
