# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a string text, replace all spaces in it with underscores, 
and if a string has more than 2 consecutive spaces, 
then replace all consecutive spaces with - 

fix_spaces("Example") == "Example"
fix_spaces("Example 1") == "Example_1"
fix_spaces(" Example 2") == "_Example_2"
fix_spaces(" Example   3") == "_Example-3"
*/
#include<stdio.h>
#include<string>
using namespace std;
string fix_spaces(string text){
    string out="";
    int spacelen=0;
    for (int i=0;i<text.length();i++)
    if (text[i]==' ') spacelen+=1;
    else
    {
        if (spacelen==1) out=out+'_';
        if (spacelen==2) out=out+"__";
        if (spacelen>2) out=out+'-';
        spacelen=0;
        out=out+text[i];
    }
    if (spacelen==1) out=out+'_';
    if (spacelen==2) out=out+"__";
    if (spacelen>2) out=out+'-';
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
Given a string text, replace all spaces in it with underscores, 
and if a string has more than 2 consecutive spaces, 
then replace all consecutive spaces with - 

fix_spaces("Example") == "Example"
fix_spaces("Example 1") == "Example_1"
fix_spaces(" Example 2") == "_Example_2"
fix_spaces(" Example   3") == "_Example-3"
*/
#include<stdio.h>
#include<string>
using namespace std;
string fix_spaces(string text){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
