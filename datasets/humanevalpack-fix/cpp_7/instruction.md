# Context

```cpp
/*
Filter an input vector of strings only for ones that contain given substring
>>> filter_by_substring({}, "a")
{}
>>> filter_by_substring({"abc", "bacd", "cde", "vector"}, "a")
{"abc", "bacd", "vector"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> filter_by_substring(vector<string> strings, string substring){
    vector<string> out;
    for (int i=0;i<strings.size();i++)
    {
        if (substring.find(strings[i])!=strings[i].npos)
        out.push_back(strings[i]);
    }
    return out;
}
```

# Instruction

Fix bugs in filter_by_substring.

# Prompt

/*
Filter an input vector of strings only for ones that contain given substring
>>> filter_by_substring({}, "a")
{}
>>> filter_by_substring({"abc", "bacd", "cde", "vector"}, "a")
{"abc", "bacd", "vector"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> filter_by_substring(vector<string> strings, string substring){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
