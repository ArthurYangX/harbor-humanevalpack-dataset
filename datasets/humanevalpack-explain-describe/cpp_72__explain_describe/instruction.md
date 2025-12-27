# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Write a function that returns true if the object q will fly, and false otherwise.
The object q will fly if it's balanced (it is a palindromic vector) and the sum of its elements is less than or equal the maximum possible weight w.

Example:
will_it_fly({1, 2}, 5) ➞ false 
// 1+2 is less than the maximum possible weight, but it's unbalanced.

will_it_fly({3, 2, 3}, 1) ➞ false
// it's balanced, but 3+2+3 is more than the maximum possible weight.

will_it_fly({3, 2, 3}, 9) ➞ true
// 3+2+3 is less than the maximum possible weight, and it's balanced.

will_it_fly({3}, 5) ➞ true
// 3 is less than the maximum possible weight, and it's balanced.
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool will_it_fly(vector<int> q,int w){
    int sum=0;
    for (int i=0;i<q.size();i++)
    {
        if (q[i]!=q[q.size()-1-i]) return false;
        sum+=q[i];
    }
    if (sum>w) return false;
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
Write a function that returns true if the object q will fly, and false otherwise.
The object q will fly if it's balanced (it is a palindromic vector) and the sum of its elements is less than or equal the maximum possible weight w.

Example:
will_it_fly({1, 2}, 5) ➞ false 
// 1+2 is less than the maximum possible weight, but it's unbalanced.

will_it_fly({3, 2, 3}, 1) ➞ false
// it's balanced, but 3+2+3 is more than the maximum possible weight.

will_it_fly({3, 2, 3}, 9) ➞ true
// 3+2+3 is less than the maximum possible weight, and it's balanced.

will_it_fly({3}, 5) ➞ true
// 3 is less than the maximum possible weight, and it's balanced.
*/
#include<stdio.h>
#include<vector>
using namespace std;
bool will_it_fly(vector<int> q,int w){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
