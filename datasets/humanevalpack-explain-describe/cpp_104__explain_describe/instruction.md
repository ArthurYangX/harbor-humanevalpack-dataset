# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a vector of positive integers x. return a sorted vector of all 
elements that hasn't any even digit.

Note: Returned vector should be sorted in increasing order.

For example:
>>> unique_digits({15, 33, 1422, 1})
{1, 15, 33}
>>> unique_digits({152, 323, 1422, 10})
{}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> unique_digits(vector<int> x){
    vector<int> out={};
    for (int i=0;i<x.size();i++)
        {
            int num=x[i];
            bool u=true;
            if (num==0) u=false;
            while (num>0 and u)
            {
                if (num%2==0) u=false;
                num=num/10;
            }
            if (u) out.push_back(x[i]);
        }
    sort(out.begin(),out.end());
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
Given a vector of positive integers x. return a sorted vector of all 
elements that hasn't any even digit.

Note: Returned vector should be sorted in increasing order.

For example:
>>> unique_digits({15, 33, 1422, 1})
{1, 15, 33}
>>> unique_digits({152, 323, 1422, 10})
{}
*/
#include<stdio.h>
#include<vector>
#include<algorithm>
using namespace std;
vector<int> unique_digits(vector<int> x){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
