# Context

```cpp
/*
Filter an input vector of strings only for ones that start with a given prefix.
>>> filter_by_prefix({}, "a")
{}
>>> filter_by_prefix({"abc", "bcd", "cde", "vector"}, "a")
{"abc", "vector"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> filter_by_prefix(vector<string> strings, string prefix){
    vector<string> out={};
    for (int i=0;i<strings.size();i++)
        if (strings[i].substr(0,prefix.length())!=prefix) out.push_back(strings[i]);
    return out;
}
```

# Instruction

Fix bugs in filter_by_prefix.

# Prompt

/*
Filter an input vector of strings only for ones that start with a given prefix.
>>> filter_by_prefix({}, "a")
{}
>>> filter_by_prefix({"abc", "bcd", "cde", "vector"}, "a")
{"abc", "vector"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> filter_by_prefix(vector<string> strings, string prefix){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
