# Context

```cpp
/*
Create a function encrypt that takes a string as an argument and
returns a string encrypted with the alphabet being rotated. 
The alphabet should be rotated in a manner such that the letters 
shift down by two multiplied to two places.
For example:
encrypt("hi") returns "lm"
encrypt("asdfghjkl") returns "ewhjklnop"
encrypt("gf") returns "kj"
encrypt("et") returns "ix"
*/
#include<stdio.h>
#include<string>
using namespace std;
string encrypt(string s){
    string out;
    int i;
    for (i=0;i<s.length();i++)
    {
        int w=((int)s[i]+4-(int)'a')%24+(int)'a';   
        out=out+(char)w;
    }
    return out;
}
```

# Instruction

Fix bugs in encrypt.

# Prompt

/*
Create a function encrypt that takes a string as an argument and
returns a string encrypted with the alphabet being rotated. 
The alphabet should be rotated in a manner such that the letters 
shift down by two multiplied to two places.
For example:
encrypt("hi") returns "lm"
encrypt("asdfghjkl") returns "ewhjklnop"
encrypt("gf") returns "kj"
encrypt("et") returns "ix"
*/
#include<stdio.h>
#include<string>
using namespace std;
string encrypt(string s){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
