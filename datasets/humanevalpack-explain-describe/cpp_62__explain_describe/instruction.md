# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
xs represent coefficients of a polynomial.
xs{0} + xs{1} * x + xs{2} * x^2 + ....
 Return derivative of this polynomial in the same form.
>>> derivative({3, 1, 2, 4, 5})
{1, 4, 12, 20}
>>> derivative({1, 2, 3})
{2, 6}
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
vector<float> derivative(vector<float> xs){
    vector<float> out={};
    for (int i=1;i<xs.size();i++)
        out.push_back(i*xs[i]);
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
xs represent coefficients of a polynomial.
xs{0} + xs{1} * x + xs{2} * x^2 + ....
 Return derivative of this polynomial in the same form.
>>> derivative({3, 1, 2, 4, 5})
{1, 4, 12, 20}
>>> derivative({1, 2, 3})
{2, 6}
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
vector<float> derivative(vector<float> xs){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
