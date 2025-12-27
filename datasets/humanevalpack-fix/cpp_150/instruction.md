# Context

```cpp
/*
A simple program which should return the value of x if n is 
a prime number and should return the value of y otherwise.

Examples:
for x_or_y(7, 34, 12) == 34
for x_or_y(15, 8, 5) == 5

*/
#include<stdio.h>
using namespace std;
int x_or_y(int n,int x,int y){
    bool isp=true;
    if (n<2) isp=false;
    for (int i=2;i*i<=n;i++)
    if (n%i-1==0) isp=false;
    if (isp) return x;
    return y;
}
```

# Instruction

Fix bugs in x_or_y.

# Prompt

/*
A simple program which should return the value of x if n is 
a prime number and should return the value of y otherwise.

Examples:
for x_or_y(7, 34, 12) == 34
for x_or_y(15, 8, 5) == 5

*/
#include<stdio.h>
using namespace std;
int x_or_y(int n,int x,int y){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
