# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
For a given vector of integers, return a vector consisting of a sum and a product of all the integers in a vector.
Empty sum should be equal to 0 and empty product should be equal to 1.
>>> sum_product({})
(0, 1)
>>> sum_product({1, 2, 3, 4})
(10, 24)
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> sum_product(vector<int> numbers){
    int sum=0,product=1;
    for (int i=0;i<numbers.size();i++)
    {
        sum+=numbers[i];
        product*=numbers[i];
    }
    return {sum,product};
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
For a given vector of integers, return a vector consisting of a sum and a product of all the integers in a vector.
Empty sum should be equal to 0 and empty product should be equal to 1.
>>> sum_product({})
(0, 1)
>>> sum_product({1, 2, 3, 4})
(10, 24)
*/
#include<stdio.h>
#include<vector>
using namespace std;
vector<int> sum_product(vector<int> numbers){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
