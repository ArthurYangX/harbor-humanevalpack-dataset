#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
>>> flip_case("Hello")
"hELLO"
*/
#include<stdio.h>
#include<string>
using namespace std;
string flip_case(string str){


// Solution Body
    string out="";
    for (int i=0;i<str.length();i++)
    {
        char w=str[i];
        if (w>=97 and w<=122) {w-=32;}
        else
            if (w>=65 and w<=90){ w+=32;}
        out=out+w;
    }
    return out;
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (flip_case("") == "");
    assert (flip_case("Hello!") == "hELLO!");
    assert (flip_case("These violent delights have violent ends") == "tHESE VIOLENT DELIGHTS HAVE VIOLENT ENDS");
}

