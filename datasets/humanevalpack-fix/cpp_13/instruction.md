# Context

```cpp
/*
Return a greatest common divisor of two integers a and b
>>> greatest_common_divisor(3, 5)
1
>>> greatest_common_divisor(25, 15)
5
*/
#include<stdio.h>
using namespace std;
int greatest_common_divisor(int a, int b){
    int out,m;
    while (true){
        if (a<b) 
        {
            m=a;a=b;b=m;
        }
        a=a%b;
        if (a==0) return a;
    }
}
```

# Instruction

Fix bugs in greatest_common_divisor.

# Prompt

/*
Return a greatest common divisor of two integers a and b
>>> greatest_common_divisor(3, 5)
1
>>> greatest_common_divisor(25, 15)
5
*/
#include<stdio.h>
using namespace std;
int greatest_common_divisor(int a, int b){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
