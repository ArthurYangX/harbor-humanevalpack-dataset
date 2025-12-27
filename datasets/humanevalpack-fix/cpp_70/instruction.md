# Context

```cpp
/*
Given vector of integers, return vector in strange order.
Strange sorting, is when you start with the minimum value,
then maximum of the remaining integers, then minimum and so on.

Examples:
strange_sort_vector({1, 2, 3, 4}) == {1, 4, 2, 3}
strange_sort_vector({5, 5, 5, 5}) == {5, 5, 5, 5}
strange_sort_vector({}) == {}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> strange_sort_list(vector<int> lst){
    vector<int> out={};
    sort(lst.begin(),lst.end());
    int l=0,r=lst.size()-1;
    while (l<r)
    {
        out.push_back(lst[l]);
        l+=2;
        out.push_back(lst[r]);
        r-=2;
    }
    if (l==r) out.push_back(lst[l]);
    return out;

}
```

# Instruction

Fix bugs in strange_sort_list.

# Prompt

/*
Given vector of integers, return vector in strange order.
Strange sorting, is when you start with the minimum value,
then maximum of the remaining integers, then minimum and so on.

Examples:
strange_sort_vector({1, 2, 3, 4}) == {1, 4, 2, 3}
strange_sort_vector({5, 5, 5, 5}) == {5, 5, 5, 5}
strange_sort_vector({}) == {}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> strange_sort_list(vector<int> lst){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
