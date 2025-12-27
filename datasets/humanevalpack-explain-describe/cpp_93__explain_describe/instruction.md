# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Write a function that takes a message, and encodes in such a 
way that it swaps case of all letters, replaces all vowels in 
the message with the letter that appears 2 places ahead of that 
vowel in the english alphabet. 
Assume only letters. 

Examples:
>>> encode('test")
"TGST"
>>> encode("This is a message")
'tHKS KS C MGSSCGG"
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string encode(string message){
    string vowels="aeiouAEIOU";
    string out="";
    for (int i=0;i<message.length();i++)
    {
        char w=message[i];
        if (w>=97 and w<=122){w=w-32;}
        else if (w>=65 and w<=90) w=w+32;
        if (find(vowels.begin(),vowels.end(),w)!=vowels.end()) w=w+2;
        out=out+w;
    }
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
Write a function that takes a message, and encodes in such a 
way that it swaps case of all letters, replaces all vowels in 
the message with the letter that appears 2 places ahead of that 
vowel in the english alphabet. 
Assume only letters. 

Examples:
>>> encode('test")
"TGST"
>>> encode("This is a message")
'tHKS KS C MGSSCGG"
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string encode(string message){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
