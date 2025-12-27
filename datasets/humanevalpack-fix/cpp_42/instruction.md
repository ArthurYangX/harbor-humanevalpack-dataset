# Context

```cpp
/*
Return vector with elements incremented by 1.
>>> incr_vector({1, 2, 3})
{2, 3, 4}
>>> incr_vector({5, 3, 5, 2, 3, 3, 9, 0, 123})
{6, 4, 6, 3, 4, 4, 10, 1, 124}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> incr_list(vector<int> l){
    for (int i=0;i<l.size();i++)
        l[i]+=2;
    return l;
}
```

# Instruction

Fix bugs in incr_list.

# Prompt

/*
Return vector with elements incremented by 1.
>>> incr_vector({1, 2, 3})
{2, 3, 4}
>>> incr_vector({5, 3, 5, 2, 3, 3, 9, 0, 123})
{6, 4, 6, 3, 4, 4, 10, 1, 124}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> incr_list(vector<int> l){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
