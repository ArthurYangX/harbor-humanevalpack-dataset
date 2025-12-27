# Context

```cpp
/*
brackets is a string of '<' and '>'.
return true if every opening bracket has a corresponding closing bracket.

>>> correct_bracketing("<")
false
>>> correct_bracketing("<>")
true
>>> correct_bracketing("<<><>>")
true
>>> correct_bracketing("><<>")
false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool correct_bracketing(string brackets){
    int level=0;
    for (int i=0;i<brackets.length();i++)
    {
        if (brackets[i]=='>') level+=1;
        if (brackets[i]=='<') level-=1;
        if (level<0) return false;
    }
    if (level!=0) return false;
    return true;
}
```

# Instruction

Fix bugs in correct_bracketing.

# Prompt

/*
brackets is a string of '<' and '>'.
return true if every opening bracket has a corresponding closing bracket.

>>> correct_bracketing("<")
false
>>> correct_bracketing("<>")
true
>>> correct_bracketing("<<><>>")
true
>>> correct_bracketing("><<>")
false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool correct_bracketing(string brackets){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
