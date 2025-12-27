# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
fibfib(0) == 0
fibfib(1) == 0
fibfib(2) == 1
fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
Please write a function to efficiently compute the n-th element of the fibfib number sequence.
>>> fibfib(1)
0
>>> fibfib(5)
4
>>> fibfib(8)
24
*/
#include<stdio.h>
using namespace std;
int fibfib(int n){
    int ff[100];
    ff[0]=0;
    ff[1]=0;
    ff[2]=1;
    for (int i=3;i<=n;i++)
        ff[i]=ff[i-1]+ff[i-2]+ff[i-3];
    return ff[n];

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
The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
fibfib(0) == 0
fibfib(1) == 0
fibfib(2) == 1
fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
Please write a function to efficiently compute the n-th element of the fibfib number sequence.
>>> fibfib(1)
0
>>> fibfib(5)
4
>>> fibfib(8)
24
*/
#include<stdio.h>
using namespace std;
int fibfib(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
