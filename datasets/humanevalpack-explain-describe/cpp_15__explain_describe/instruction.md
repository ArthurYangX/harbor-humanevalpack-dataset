# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Return a string containing space-delimited numbers starting from 0 upto n inclusive.
>>> string_sequence(0)
"0"
>>> string_sequence(5)
"0 1 2 3 4 5"
*/
#include<stdio.h>
#include<string>
using namespace std;
string string_sequence(int n){
    string out="0";
    for (int i=1;i<=n;i++)
    out=out+" "+to_string(i);
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
Return a string containing space-delimited numbers starting from 0 upto n inclusive.
>>> string_sequence(0)
"0"
>>> string_sequence(5)
"0 1 2 3 4 5"
*/
#include<stdio.h>
#include<string>
using namespace std;
string string_sequence(int n){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
