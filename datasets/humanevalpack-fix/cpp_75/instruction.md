# Context

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
    for (int i=0;i*i<=a;i++)
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

Fix bugs in is_multiply_prime.

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

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
