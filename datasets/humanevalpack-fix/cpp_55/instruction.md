# Context

```cpp
/*
Return n-th Fibonacci number.
>>> fib(10)
55
>>> fib(1)
1
>>> fib(8)
21
*/
#include<stdio.h>
using namespace std;
int fib(int n){
    int f[1000];
    f[0]=0;f[1]=1;f[2]=2;
    for (int i=3;i<=n; i++)
    f[i]=f[i-1]+f[i-2];
    return f[n];
}
```

# Instruction

Fix bugs in fib.

# Prompt

/*
Return n-th Fibonacci number.
>>> fib(10)
55
>>> fib(1)
1
>>> fib(8)
21
*/
#include<stdio.h>
using namespace std;
int fib(int n){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
