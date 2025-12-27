# Context

```cpp
/*
pairs_sum_to_zero takes a vector of integers as an input.
it returns true if there are two distinct elements in the vector that
sum to zero, and false otherwise.
>>> pairs_sum_to_zero({1, 3, 5, 0})
false
>>> pairs_sum_to_zero({1, 3, -2, 1})
false
>>> pairs_sum_to_zero({1, 2, 3, 7})
false
>>> pairs_sum_to_zero({2, 4, -5, 3, 5, 7})
true
>>> pairs_sum_to_zero({1})
false
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool pairs_sum_to_zero(vector<int> l){
    for (int i=0;i<l.size();i++)
    for (int j=i;j<l.size();j++)
    if (l[i]+l[j]==0) return true;
    return false;
}
```

# Instruction

Fix bugs in pairs_sum_to_zero.

# Prompt

/*
pairs_sum_to_zero takes a vector of integers as an input.
it returns true if there are two distinct elements in the vector that
sum to zero, and false otherwise.
>>> pairs_sum_to_zero({1, 3, 5, 0})
false
>>> pairs_sum_to_zero({1, 3, -2, 1})
false
>>> pairs_sum_to_zero({1, 2, 3, 7})
false
>>> pairs_sum_to_zero({2, 4, -5, 3, 5, 7})
true
>>> pairs_sum_to_zero({1})
false
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool pairs_sum_to_zero(vector<int> l){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
