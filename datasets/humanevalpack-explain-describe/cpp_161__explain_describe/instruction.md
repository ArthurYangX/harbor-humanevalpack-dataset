# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You are given a string s.
if s[i] is a letter, reverse its case from lower to upper or vise versa, 
otherwise keep it as it is.
If the string contains no letters, reverse the string.
The function should return the resulted string.
Examples
solve("1234") = "4321"
solve("ab") = "AB"
solve("#a@C") = "#A@c"
*/
#include<stdio.h>
#include<string>
using namespace std;
string solve(string s){
    int nletter=0;
    string out="";
    for (int i=0;i<s.length();i++)
    {
        char w=s[i];
        if (w>=65 and w<=90) w=w+32;
        else if (w>=97 and w<=122) w=w-32;
        else nletter+=1;
        out=out+w;
    }
    if (nletter==s.length())
    {
        string p(s.rbegin(),s.rend());
        return p;
    }
    else return out;
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
if s[i] is a letter, reverse its case from lower to upper or vise versa, 
otherwise keep it as it is.
If the string contains no letters, reverse the string.
The function should return the resulted string.
Examples
solve("1234") = "4321"
solve("ab") = "AB"
solve("#a@C") = "#A@c"
*/
#include<stdio.h>
#include<string>
using namespace std;
string solve(string s){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
