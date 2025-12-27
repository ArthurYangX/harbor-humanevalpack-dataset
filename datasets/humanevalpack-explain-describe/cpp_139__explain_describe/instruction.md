# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
The Brazilian factorial is defined as:
brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
where n > 0

For example:
>>> special_factorial(4)
288

The function will receive an integer as input and should return the special
factorial of this integer.
*/
#include<stdio.h>
using namespace std;
long long special_factorial(int n){
    long long fact=1,bfact=1;
    for (int i=1;i<=n;i++)
    {
        fact=fact*i;
        bfact=bfact*fact;
    }
    return bfact;
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
The Brazilian factorial is defined as:
brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
where n > 0

For example:
>>> special_factorial(4)
288

The function will receive an integer as input and should return the special
factorial of this integer.
*/
#include<stdio.h>
using namespace std;
long long special_factorial(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
