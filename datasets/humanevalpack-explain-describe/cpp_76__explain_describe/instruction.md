# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Your task is to write a function that returns true if a number x is a simple
power of n and false in other cases.
x is a simple power of n if n**int=x
For example:
is_simple_power(1, 4) => true
is_simple_power(2, 2) => true
is_simple_power(8, 2) => true
is_simple_power(3, 2) => false
is_simple_power(3, 1) => false
is_simple_power(5, 3) => false
*/
#include<stdio.h>
#include<math.h>
using namespace std;
bool is_simple_power(int x,int n){
    int p=1,count=0;
    while (p<=x and count<100)
    {
        if (p==x) return true;
        p=p*n;count+=1;
    }
    return false;
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
Your task is to write a function that returns true if a number x is a simple
power of n and false in other cases.
x is a simple power of n if n**int=x
For example:
is_simple_power(1, 4) => true
is_simple_power(2, 2) => true
is_simple_power(8, 2) => true
is_simple_power(3, 2) => false
is_simple_power(3, 1) => false
is_simple_power(5, 3) => false
*/
#include<stdio.h>
#include<math.h>
using namespace std;
bool is_simple_power(int x,int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
