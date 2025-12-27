# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
You are given a word. Your task is to find the closest vowel that stands between 
two consonants from the right side of the word (case sensitive).

Vowels in the beginning and ending doesn't count. Return empty string if you didn't
find any vowel met the above condition. 

You may assume that the given string contains English letter only.

Example:
get_closest_vowel("yogurt") ==> "u"
get_closest_vowel("FULL") ==> "U"
get_closest_vowel("quick") ==> ""
get_closest_vowel("ab") ==> ""
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string get_closest_vowel(string word){
    string out="";
    string vowels="AEIOUaeiou";
    for (int i=word.length()-2;i>=1;i-=1)
        if (find(vowels.begin(),vowels.end(),word[i])!=vowels.end())
            if (find(vowels.begin(),vowels.end(),word[i+1])==vowels.end())
                if (find(vowels.begin(),vowels.end(),word[i-1])==vowels.end())
                    return out+word[i];
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
You are given a word. Your task is to find the closest vowel that stands between 
two consonants from the right side of the word (case sensitive).

Vowels in the beginning and ending doesn't count. Return empty string if you didn't
find any vowel met the above condition. 

You may assume that the given string contains English letter only.

Example:
get_closest_vowel("yogurt") ==> "u"
get_closest_vowel("FULL") ==> "U"
get_closest_vowel("quick") ==> ""
get_closest_vowel("ab") ==> ""
*/
#include<stdio.h>
#include<string>
#include<algorithm>
using namespace std;
string get_closest_vowel(string word){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
