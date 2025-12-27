# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
The Fib4 number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
fib4(0) -> 0
fib4(1) -> 0
fib4(2) -> 2
fib4(3) -> 0
fib4(n) -> fib4(n-1) + fib4(n-2) + fib4(n-3) + fib4(n-4).
Please write a function to efficiently compute the n-th element of the fib4 number sequence.  Do not use recursion.
>>> fib4(5)
4
>>> fib4(6)
8
>>> fib4(7)
14
*/
#include<stdio.h>
using namespace std;
int fib4(int n){
    int f[100];
    f[0]=0;
    f[1]=0;
    f[2]=2;
    f[3]=0;
    for (int i=4;i<=n;i++)
    {
        f[i]=f[i-1]+f[i-2]+f[i-3]+f[i-4];
    }
    return f[n];
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
The Fib4 number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
fib4(0) -> 0
fib4(1) -> 0
fib4(2) -> 2
fib4(3) -> 0
fib4(n) -> fib4(n-1) + fib4(n-2) + fib4(n-3) + fib4(n-4).
Please write a function to efficiently compute the n-th element of the fib4 number sequence.  Do not use recursion.
>>> fib4(5)
4
>>> fib4(6)
8
>>> fib4(7)
14
*/
#include<stdio.h>
using namespace std;
int fib4(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
