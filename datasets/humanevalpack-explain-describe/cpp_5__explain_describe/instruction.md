# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Insert a number "delimeter" between every two consecutive elements of input vector `numbers"
>>> intersperse({}, 4)
{}
>>> intersperse({1, 2, 3}, 4)
{1, 4, 2, 4, 3}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> intersperse(vector<int> numbers, int delimeter){
    vector<int> out={};
    if (numbers.size()>0) out.push_back(numbers[0]);
    for (int i=1;i<numbers.size();i++)
    {
        out.push_back(delimeter);
        out.push_back(numbers[i]);

    }
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
Insert a number "delimeter" between every two consecutive elements of input vector `numbers"
>>> intersperse({}, 4)
{}
>>> intersperse({1, 2, 3}, 4)
{1, 4, 2, 4, 3}
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> intersperse(vector<int> numbers, int delimeter){ 


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
