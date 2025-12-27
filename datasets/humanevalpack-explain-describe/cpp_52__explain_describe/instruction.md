# Context

You are given a reference implementation (canonical solution) to explain.

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
        if (l[i]>=t) return false;
    return true;
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

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
