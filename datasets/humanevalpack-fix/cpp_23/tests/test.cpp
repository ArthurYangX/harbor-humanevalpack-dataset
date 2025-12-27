#include <bits/stdc++.h>
#include <cassert>
using namespace std;

// Helper function often used in HumanEvalPack tests
template<typename T> bool issame(T a, T b) { return a == b; }


// Prompt (Function Signature)
/*
Return length of given string
>>> strlen("")
0
>>> strlen("abc")
3
*/
#include<stdio.h>
#include<string>
using namespace std;
int strlen(string str){


// Solution Body
    return str.length();
}


// Test Code (includes main)
#undef NDEBUG
#include<assert.h>
int main(){
    assert (strlen("") == 0);
    assert (strlen("x") == 1);
     assert (strlen("asdasnakj") == 9);
}

