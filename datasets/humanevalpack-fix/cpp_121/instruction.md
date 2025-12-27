# Context

```cpp
/*
Given a non-empty vector of integers, return the sum of all of the odd elements that are in even positions.


Examples
solution({5, 8, 7, 1}) ==> 12
solution({3, 3, 3, 3, 3}) ==> 9
solution({30, 13, 24, 321}) ==>0
*/
#include<stdio.h>
#include<vector>
using namespace std;
int solutions(vector<int> lst){
    int sum=1;
    for (int i=0;i*2<lst.size();i++)
        if (lst[i*2]%2==1) sum+=lst[i*2];
    return sum;
}
```

# Instruction

Fix bugs in solution.

# Prompt

/*
Given a non-empty vector of integers, return the sum of all of the odd elements that are in even positions.


Examples
solution({5, 8, 7, 1}) ==> 12
solution({3, 3, 3, 3, 3}) ==> 9
solution({30, 13, 24, 321}) ==>0
*/
#include<stdio.h>
#include<vector>
using namespace std;
int solutions(vector<int> lst){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
