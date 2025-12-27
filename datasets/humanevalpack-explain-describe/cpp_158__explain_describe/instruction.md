# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Write a function that accepts a vector of strings.
The vector contains different words. Return the word with maximum number
of unique characters. If multiple strings have maximum number of unique
characters, return the one which comes first in lexicographical order.

find_max({"name", "of", 'string"}) == 'string"
find_max({"name", "enam", "game"}) == "enam"
find_max({"aaaaaaa", "bb" ,"cc"}) == "aaaaaaa"
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
string find_max(vector<string> words){
    string max="";
    int maxu=0;
    for (int i=0;i<words.size();i++)
    {
        string unique="";
        for (int j=0;j<words[i].length();j++)
            if (find(unique.begin(),unique.end(),words[i][j])==unique.end())
                unique=unique+words[i][j];
        if (unique.length()>maxu or (unique.length()==maxu and words[i]<max))
        {
            max=words[i];
            maxu=unique.length();
        }
    }
    return max;
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
Write a function that accepts a vector of strings.
The vector contains different words. Return the word with maximum number
of unique characters. If multiple strings have maximum number of unique
characters, return the one which comes first in lexicographical order.

find_max({"name", "of", 'string"}) == 'string"
find_max({"name", "enam", "game"}) == "enam"
find_max({"aaaaaaa", "bb" ,"cc"}) == "aaaaaaa"
*/
#include<stdio.h>
#include<vector>
#include<string>
#include<algorithm>
using namespace std;
string find_max(vector<string> words){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
