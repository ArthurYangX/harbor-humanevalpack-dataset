# Context

```cpp
/*
For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
>>> flip_case("Hello")
"hELLO"
*/
#include<stdio.h>
#include<string>
using namespace std;
string flip_case(string str){
    string out="";
    for (int i=0;i<str.length();i++)
    {
        char w=str[i];
        if (w>=97 and w<=122) {w+=32;}
        else
            if (w>=65 and w<=90){ w-=32;}
        out=out+w;
    }
    return out;
}
```

# Instruction

Fix bugs in flip_case.

# Prompt

/*
For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
>>> flip_case("Hello")
"hELLO"
*/
#include<stdio.h>
#include<string>
using namespace std;
string flip_case(string str){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
