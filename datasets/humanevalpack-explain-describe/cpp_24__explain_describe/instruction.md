# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
For a given number n, find the largest number that divides n evenly, smaller than n
>>> largest_divisor(15)
5
*/
#include<stdio.h>
using namespace std;
int largest_divisor(int n){
    for (int i=2;i*i<=n;i++)
        if (n%i==0) return  n/i;
    return 1;

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
For a given number n, find the largest number that divides n evenly, smaller than n
>>> largest_divisor(15)
5
*/
#include<stdio.h>
using namespace std;
int largest_divisor(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
