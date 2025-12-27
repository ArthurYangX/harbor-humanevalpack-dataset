# Context

```cpp
/*
Complete the function that takes two integers and returns 
the product of their unit digits.
Assume the input is always valid.
Examples:
multiply(148, 412) should return 16.
multiply(19, 28) should return 72.
multiply(2020, 1851) should return 0.
multiply(14,-15) should return 20.
*/
#include<stdio.h>
#include<math.h>
using namespace std;
int multiply(int a,int b){
    return (abs(a)%10)*(abs(b)%10)*a*b;
}
```

# Instruction

Fix bugs in multiply.

# Prompt

/*
Complete the function that takes two integers and returns 
the product of their unit digits.
Assume the input is always valid.
Examples:
multiply(148, 412) should return 16.
multiply(19, 28) should return 72.
multiply(2020, 1851) should return 0.
multiply(14,-15) should return 20.
*/
#include<stdio.h>
#include<math.h>
using namespace std;
int multiply(int a,int b){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
