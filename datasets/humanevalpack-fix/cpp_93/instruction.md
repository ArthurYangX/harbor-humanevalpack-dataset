# Context

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
    string vowels="aeiou";
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

Fix bugs in encode.

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

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
