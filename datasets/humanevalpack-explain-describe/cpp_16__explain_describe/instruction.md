# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a string, find out how many distinct characters (regardless of case) does it consist of
>>> count_distinct_characters("xyzXYZ")
3
>>> count_distinct_characters("Jerry")
4
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
int count_distinct_characters(string str){
    vector<char> distinct={};
    transform(str.begin(),str.end(),str.begin(),::tolower);
    for (int i=0;i<str.size();i++)
    {
        bool isin=false;
        for (int j=0;j<distinct.size();j++)
            if (distinct[j]==str[i])
                isin=true;
        if (isin==false) distinct.push_back(str[i]);

    }
    return distinct.size();
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
Given a string, find out how many distinct characters (regardless of case) does it consist of
>>> count_distinct_characters("xyzXYZ")
3
>>> count_distinct_characters("Jerry")
4
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
int count_distinct_characters(string str){ 


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
