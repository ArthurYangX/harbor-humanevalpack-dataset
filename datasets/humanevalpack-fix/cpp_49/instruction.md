# Context

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
    int out=0;
    for (int i=0;i<n;i++)
        out=(out*2)%p;
    return out;
}
```

# Instruction

Fix bugs in modp.

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

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
