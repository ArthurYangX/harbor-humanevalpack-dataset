# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
remove_vowels is a function that takes string and returns string without vowels.
>>> remove_vowels("")
""
>>> remove_vowels("abcdef\nghijklm")
"bcdf\nghjklm"
>>> remove_vowels("abcdef")
"bcdf"
>>> remove_vowels("aaaaa")
""
>>> remove_vowels("aaBAA")
"B"
>>> remove_vowels("zbcd")
"zbcd"
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string remove_vowels(string text){
    string out="";
    string vowels="AEIOUaeiou";
    for (int i=0;i<text.length();i++)
        if (find(vowels.begin(),vowels.end(),text[i])==vowels.end())
            out=out+text[i];
    return out;

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
remove_vowels is a function that takes string and returns string without vowels.
>>> remove_vowels("")
""
>>> remove_vowels("abcdef\nghijklm")
"bcdf\nghjklm"
>>> remove_vowels("abcdef")
"bcdf"
>>> remove_vowels("aaaaa")
""
>>> remove_vowels("aaBAA")
"B"
>>> remove_vowels("zbcd")
"zbcd"
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string remove_vowels(string text){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
