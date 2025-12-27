# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Return maximum element in the vector.
>>> max_element({1, 2, 3})
3
>>> max_element({5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10})
123
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
float max_element(vector<float> l){
  float max=-10000;
  for (int i=0;i<l.size();i++)
  if (max<l[i]) max=l[i];
  return max;

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
Return maximum element in the vector.
>>> max_element({1, 2, 3})
3
>>> max_element({5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10})
123
*/
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
float max_element(vector<float> l){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
