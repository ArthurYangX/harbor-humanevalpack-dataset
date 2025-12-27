# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a vector arr of integers, find the minimum number of elements that
need to be changed to make the vector palindromic. A palindromic vector is a vector that
is read the same backwards and forwards. In one change, you can change one element to any other element.

For example:
smallest_change({1,2,3,5,4,7,9,6}) == 4
smallest_change({1, 2, 3, 4, 3, 2, 2}) == 1
smallest_change({1, 2, 3, 2, 1}) == 0
*/
#include<stdio.h>
#include<vector>
using namespace std;
int smallest_change(vector<int> arr){
    int out=0;
    for (int i=0;i<arr.size()-1-i;i++)
        if (arr[i]!=arr[arr.size()-1-i])
            out+=1;
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
Given a vector arr of integers, find the minimum number of elements that
need to be changed to make the vector palindromic. A palindromic vector is a vector that
is read the same backwards and forwards. In one change, you can change one element to any other element.

For example:
smallest_change({1,2,3,5,4,7,9,6}) == 4
smallest_change({1, 2, 3, 4, 3, 2, 2}) == 1
smallest_change({1, 2, 3, 2, 1}) == 0
*/
#include<stdio.h>
#include<vector>
using namespace std;
int smallest_change(vector<int> arr){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
