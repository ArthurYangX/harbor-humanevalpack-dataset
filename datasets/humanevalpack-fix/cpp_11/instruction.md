# Context

```cpp
/*
Input are two strings a and b consisting only of 1s and 0s.
Perform binary XOR on these inputs and return result also as a string.
>>> string_xor("010", "110")
"100"
*/
#include<stdio.h>
#include<string>
using namespace std;
string string_xor(string a,string b){
    string output="";
    for (int i=0;(i<a.length() and i<b.length());i++)
    {
        if (i<a.length() and i<b.length())
        {
            if (a[i]== b[i]) 
            {
                output+='1';
            }  
            else output+='0';
        }
        else
        {
            if (i>=a.length()) 
            {
            output+=b[i];
            }
            else output+=a[i];
        }
    }
    return output;
}
```

# Instruction

Fix bugs in string_xor.

# Prompt

/*
Input are two strings a and b consisting only of 1s and 0s.
Perform binary XOR on these inputs and return result also as a string.
>>> string_xor("010", "110")
"100"
*/
#include<stdio.h>
#include<string>
using namespace std;
string string_xor(string a,string b){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
