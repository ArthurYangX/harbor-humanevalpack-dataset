# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Return 2^n modulo p (be aware of numerics).
>>> modp(3, 5)
3
>>> modp(1101, 101)
2
>>> modp(0, 101)
1
>>> modp(3, 11)
8
>>> modp(100, 101)
1
*/
#include<stdio.h>
using namespace std;
int modp(int n,int p){
    int out=1;
    for (int i=0;i<n;i++)
        out=(out*2)%p;
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
Return 2^n modulo p (be aware of numerics).
>>> modp(3, 5)
3
>>> modp(1101, 101)
2
>>> modp(0, 101)
1
>>> modp(3, 11)
8
>>> modp(100, 101)
1
*/
#include<stdio.h>
using namespace std;
int modp(int n,int p){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
