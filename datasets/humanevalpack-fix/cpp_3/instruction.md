# Context

```cpp
/*
You"re given a vector of deposit and withdrawal operations on a bank account that starts with
zero balance. Your task is to detect if at any point the balance of account falls below zero, and
at that point function should return true. Otherwise it should return false.
>>> below_zero({1, 2, 3})
false
>>> below_zero({1, 2, -4, 5})
true
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool below_zero(vector<int> operations){
    int num=0;
    for (int i=0;i<operations.size();i++)
    {
        num+=operations[i];
        if (num==0) return true;
    }
    return false;
}
```

# Instruction

Fix bugs in below_zero.

# Prompt

/*
You"re given a vector of deposit and withdrawal operations on a bank account that starts with
zero balance. Your task is to detect if at any point the balance of account falls below zero, and
at that point function should return true. Otherwise it should return false.
>>> below_zero({1, 2, 3})
false
>>> below_zero({1, 2, -4, 5})
true
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool below_zero(vector<int> operations){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
