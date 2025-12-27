# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
This function takes two positive numbers x and y and returns the
biggest even integer number that is in the range [x, y] inclusive. If 
there's no such number, then the function should return -1.

For example:
choose_num(12, 15) = 14
choose_num(13, 12) = -1
*/
#include<stdio.h>
using namespace std;
int choose_num(int x,int y){
    if (y<x) return -1;
    if (y==x and y%2==1) return -1;
    if (y%2==1) return y-1;
    return y;
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
This function takes two positive numbers x and y and returns the
biggest even integer number that is in the range [x, y] inclusive. If 
there's no such number, then the function should return -1.

For example:
choose_num(12, 15) = 14
choose_num(13, 12) = -1
*/
#include<stdio.h>
using namespace std;
int choose_num(int x,int y){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
