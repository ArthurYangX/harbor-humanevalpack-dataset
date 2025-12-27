# Context

```cpp
/*
Concatenate vector of strings into a single string
>>> concatenate({})
""
>>> concatenate({"a", "b", "c"})
"abc"
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
string concatenate(vector<string> strings){
    string out=" ";
    for (int i=0;i<strings.size();i++)
        out=out+strings[i];
    return out;
}
```

# Instruction

Fix bugs in concatenate.

# Prompt

/*
Concatenate vector of strings into a single string
>>> concatenate({})
""
>>> concatenate({"a", "b", "c"})
"abc"
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
string concatenate(vector<string> strings){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
