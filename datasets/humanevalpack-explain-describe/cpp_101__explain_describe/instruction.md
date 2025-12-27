# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You will be given a string of words separated by commas or spaces. Your task is
to split the string into words and return a vector of the words.

For example:
words_string("Hi, my name is John") == {"Hi", "my", "name", "is", "John"}
words_string("One, two, three, four, five, six") == {"One", 'two", 'three", "four", "five", 'six"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> words_string(string s){
    string current="";
    vector<string> out={};
    s=s+' ';
    for (int i=0;i<s.length();i++)
     if (s[i]==' ' or s[i]==',')
     {
        if (current.length()>0)
        {
            out.push_back(current);
            current="";
        }
     }
     else current=current+s[i];
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
You will be given a string of words separated by commas or spaces. Your task is
to split the string into words and return a vector of the words.

For example:
words_string("Hi, my name is John") == {"Hi", "my", "name", "is", "John"}
words_string("One, two, three, four, five, six") == {"One", 'two", 'three", "four", "five", 'six"}
*/
#include<stdio.h>
#include<vector>
#include<string>
using namespace std;
vector<string> words_string(string s){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
