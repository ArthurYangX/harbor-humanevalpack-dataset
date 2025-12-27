# Context

```cpp
/*
Return length of given string
>>> strlen("")
0
>>> strlen("abc")
3
*/
#include<stdio.h>
#include<string>
using namespace std;
int strlen(string str){
    return str.length() - 1;
}
```

# Instruction

Fix bugs in strlen.

# Prompt

/*
Return length of given string
>>> strlen("")
0
>>> strlen("abc")
3
*/
#include<stdio.h>
#include<string>
using namespace std;
int strlen(string str){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
