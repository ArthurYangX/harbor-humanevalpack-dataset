# Context

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

Fix bugs in count_distinct_characters.

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

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
