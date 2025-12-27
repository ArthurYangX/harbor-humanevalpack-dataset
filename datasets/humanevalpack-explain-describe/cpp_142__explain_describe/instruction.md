# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
"
This function will take a vector of integers. For all entries in the vector, the function shall square the integer entry if its index is a 
multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
change the entries in the vector whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 

Examples:
For lst = {1,2,3} the output should be 6
For lst = {}  the output should be 0
For lst = {-1,-5,2,-1,-5}  the output should be -126
*/
#include<stdio.h>
#include<vector>
using namespace std;
int sum_squares(vector<int> lst){
    int sum=0;
    for (int i=0;i<lst.size();i++)
        if (i%3==0) sum+=lst[i]*lst[i];
        else if (i%4==0) sum+=lst[i]*lst[i]*lst[i];
        else sum+=lst[i];
    return sum;
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
"
This function will take a vector of integers. For all entries in the vector, the function shall square the integer entry if its index is a 
multiple of 3 and will cube the integer entry if its index is a multiple of 4 and not a multiple of 3. The function will not 
change the entries in the vector whose indexes are not a multiple of 3 or 4. The function shall then return the sum of all entries. 

Examples:
For lst = {1,2,3} the output should be 6
For lst = {}  the output should be 0
For lst = {-1,-5,2,-1,-5}  the output should be -126
*/
#include<stdio.h>
#include<vector>
using namespace std;
int sum_squares(vector<int> lst){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
