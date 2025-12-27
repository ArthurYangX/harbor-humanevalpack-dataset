# Context

```cpp
/*
Task
Write a function that takes a string as input and returns the sum of the upper characters only's
ASCII codes.

Examples:
    digitSum("") => 0
    digitSum("abAB") => 131
    digitSum("abcCd") => 67
    digitSum("helloE") => 69
    digitSum("woArBld") => 131
    digitSum("aAaaaXa") => 153
*/
#include<stdio.h>
#include<string>
using namespace std;
int digitSum(string s){
    int sum=0;
    for (int i=0;i<s.length();i++)
        if (s[i]>=65 and s[i]<=100)
            sum+=s[i];
    return sum;
}
```

# Instruction

Fix bugs in digitSum.

# Prompt

/*
Task
Write a function that takes a string as input and returns the sum of the upper characters only's
ASCII codes.

Examples:
    digitSum("") => 0
    digitSum("abAB") => 131
    digitSum("abcCd") => 67
    digitSum("helloE") => 69
    digitSum("woArBld") => 131
    digitSum("aAaaaXa") => 153
*/
#include<stdio.h>
#include<string>
using namespace std;
int digitSum(string s){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
