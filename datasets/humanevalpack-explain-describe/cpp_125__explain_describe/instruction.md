# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given a string of words, return a vector of words split on whitespace, if no whitespaces exists in the text you
should split on commas ',' if no commas exists you should return a vector with one element, the number of lower-case letters with odd order in the
alphabet, ord("a") = 0, ord("b") = 1, ... ord("z") = 25
Examples
split_words("Hello world!") ➞ {"Hello", "world!"}
split_words("Hello,world!") ➞ {"Hello", "world!"}
split_words("abcdef") == {"3"} 
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
vector<string> split_words(string txt){
    int i;
    string current="";
    vector<string> out={};
    if (find(txt.begin(),txt.end(),' ')!=txt.end())
    {
        txt=txt+' ';
        for (i=0;i<txt.length();i++)
            if (txt[i]==' ') 
            {
                if (current.length()>0)out.push_back(current); 
                current="";
            }
            else current=current+txt[i];
        return out;
    }
    if (find(txt.begin(),txt.end(),',')!=txt.end())
    {
        txt=txt+',';
        for (i=0;i<txt.length();i++)
            if (txt[i]==',') 
            {
                if (current.length()>0)out.push_back(current); 
                current="";
            }
            else current=current+txt[i];
        return out;
    }
    int num=0;
    for (i=0;i<txt.length();i++)
        if (txt[i]>=97 and txt[i]<=122 and txt[i]%2==0)
            num+=1;
    return {to_string(num)};
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
Given a string of words, return a vector of words split on whitespace, if no whitespaces exists in the text you
should split on commas ',' if no commas exists you should return a vector with one element, the number of lower-case letters with odd order in the
alphabet, ord("a") = 0, ord("b") = 1, ... ord("z") = 25
Examples
split_words("Hello world!") ➞ {"Hello", "world!"}
split_words("Hello,world!") ➞ {"Hello", "world!"}
split_words("abcdef") == {"3"} 
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
vector<string> split_words(string txt){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
