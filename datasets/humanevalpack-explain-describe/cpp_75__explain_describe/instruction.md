# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Write a function that returns true if the given number is the multiplication of 3 prime numbers
and false otherwise.
Knowing that (a) is less then 100. 
Example:
is_multiply_prime(30) == true
30 = 2 * 3 * 5
*/
#include<stdio.h>
using namespace std;
bool is_multiply_prime(int a){
    int num=0;
    for (int i=2;i*i<=a;i++)
    while (a%i==0 and a>i)
    {
        a=a/i;
        num+=1;
    }
    if (num==2) return true;
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
Write a function that returns true if the given number is the multiplication of 3 prime numbers
and false otherwise.
Knowing that (a) is less then 100. 
Example:
is_multiply_prime(30) == true
30 = 2 * 3 * 5
*/
#include<stdio.h>
using namespace std;
bool is_multiply_prime(int a){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
