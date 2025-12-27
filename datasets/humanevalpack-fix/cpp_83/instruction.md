# Context

```cpp
/*
Given a positive integer n, return the count of the numbers of n-digit
positive integers that start or end with 1.
*/
#include<stdio.h>
using namespace std;
int starts_one_ends(int n){
    if (n<1) return 0;
    if (n==1) return 1;
    int out=18;
    for (int i=2;i<n;i++)
        out=out*i*10;
    return out;
}
```

# Instruction

Fix bugs in starts_one_ends.

# Prompt

/*
Given a positive integer n, return the count of the numbers of n-digit
positive integers that start or end with 1.
*/
#include<stdio.h>
using namespace std;
int starts_one_ends(int n){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
