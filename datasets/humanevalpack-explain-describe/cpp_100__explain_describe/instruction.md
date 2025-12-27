# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a positive integer n, you have to make a pile of n levels of stones.
The first level has n stones.
The number of stones in the next level is:
    - the next odd number if n is odd.
    - the next even number if n is even.
Return the number of stones in each level in a vector, where element at index
i represents the number of stones in the level (i+1).

Examples:
>>> make_a_pile(3)
{3, 5, 7}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> make_a_pile(int n){
    vector<int> out={n};
    for (int i=1;i<n;i++)
        out.push_back(out[out.size()-1]+2);
    return out;
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
Given a positive integer n, you have to make a pile of n levels of stones.
The first level has n stones.
The number of stones in the next level is:
    - the next odd number if n is odd.
    - the next even number if n is even.
Return the number of stones in each level in a vector, where element at index
i represents the number of stones in the level (i+1).

Examples:
>>> make_a_pile(3)
{3, 5, 7}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> make_a_pile(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
