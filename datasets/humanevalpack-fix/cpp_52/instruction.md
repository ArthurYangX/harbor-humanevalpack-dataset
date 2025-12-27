# Context

```cpp
/*
Return true if all numbers in the vector l are below threshold t.
>>> below_threshold({1, 2, 4, 10}, 100)
true
>>> below_threshold({1, 20, 4, 10}, 5)
false
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool below_threshold(vector<int>l, int t){
    for (int i=0;i<l.size();i++)
        if (l[i]>=t) return true;
    return false;
}
```

# Instruction

Fix bugs in below_threshold.

# Prompt

/*
Return true if all numbers in the vector l are below threshold t.
>>> below_threshold({1, 2, 4, 10}, 100)
true
>>> below_threshold({1, 20, 4, 10}, 5)
false
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool below_threshold(vector<int>l, int t){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
