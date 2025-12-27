#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
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


// Solution Body
    string out="0";
    for (int i=1;i<=n;i++)
    out=out+" "+to_string(i);
    return out;
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (string_sequence(0) == "0");
    assert (string_sequence(3) == "0 1 2 3");
     assert (string_sequence(10) == "0 1 2 3 4 5 6 7 8 9 10");
}

