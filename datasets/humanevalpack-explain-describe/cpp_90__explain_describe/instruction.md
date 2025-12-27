# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You are given a vector of integers.
Write a function next_smallest() that returns the 2nd smallest element of the vector.
Return None if there is no such element.

next_smallest({1, 2, 3, 4, 5}) == 2
next_smallest({5, 1, 4, 3, 2}) == 2
next_smallest({}) == None
next_smallest({1, 1}) == None
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
int next_smallest(vector<int> lst){
    sort(lst.begin(),lst.end());
    for (int i=1;i<lst.size();i++)
        if (lst[i]!=lst[i-1]) return lst[i];
    return -1;
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

/*
You are given a vector of integers.
Write a function next_smallest() that returns the 2nd smallest element of the vector.
Return None if there is no such element.

next_smallest({1, 2, 3, 4, 5}) == 2
next_smallest({5, 1, 4, 3, 2}) == 2
next_smallest({}) == None
next_smallest({1, 1}) == None
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
int next_smallest(vector<int> lst){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
