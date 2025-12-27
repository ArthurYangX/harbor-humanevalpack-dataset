#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
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


// Solution Body
    string out="";
    for (int i=0;i<strings.size();i++)
        out=out+strings[i];
    return out;
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (concatenate({}) == "");
     assert (concatenate({"x", "y", "z"}) == "xyz");
    assert (concatenate({"x", "y", "z", "w", "k"}) == "xyzwk");
}

