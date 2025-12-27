# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You'll be given a string of words, and your task is to count the number
of boredoms. A boredom is a sentence that starts with the word "I".
Sentences are delimited by '.', '?' or '!'.

For example:
>>> is_bored("Hello world")
0
>>> is_bored("The sky is blue. The sun is shining. I love this weather")
1
*/
#include<stdio.h>
#include<string>
using namespace std;
int is_bored(string S){
    bool isstart=true;
    bool isi=false;
    int sum=0;
    for (int i=0;i<S.length();i++)
    {
        if (S[i]==' ' and isi) {isi=false; sum+=1;}
        if (S[i]=='I' and isstart) {isi=true;  }
        else isi=false;   
        if (S[i]!=' ') { isstart=false;}
        if (S[i]=='.' or S[i]=='?' or S[i]=='!') isstart=true;
    }
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
You'll be given a string of words, and your task is to count the number
of boredoms. A boredom is a sentence that starts with the word "I".
Sentences are delimited by '.', '?' or '!'.

For example:
>>> is_bored("Hello world")
0
>>> is_bored("The sky is blue. The sun is shining. I love this weather")
1
*/
#include<stdio.h>
#include<string>
using namespace std;
int is_bored(string S){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
