# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
double poly(vector<double> xs, double x){
    /* 
    Evaluates polynomial with coefficients xs at point x. return xs[0] + xs[1] * x + xs[1] * x^2 + .... xs[n] * x^n 
    */
    double sum=0;
    int i;
    for (i=0;i<xs.size();i++)
    {
        sum+=xs[i]*pow(x,i);
    }
    return sum;
}

double find_zero(vector<double> xs){
    /*
    xs are coefficients of a polynomial. find_zero find x such that poly(x) = 0. find_zero returns only only zero point, even if there are many. 
    Moreover, find_zero only takes list xs having even number of coefficients and largest non zero coefficient as it guarantees a solution.
    >>> round(find_zero([1, 2]), 2) #f(x) = 1 + 2x 
    -0.5 
    >>> round(find_zero([-6, 11, -6, 1]), 2) # (x - 1) * (x - 2) * (x - 3) = -6 + 11x - 6x^2 + x^3 
    1.0
    */
    double ans=0;
    double value;
    value=poly(xs,ans);
    while (abs(value)>1e-6)
    {
        double driv=0;
        for (int i=1;i<xs.size();i++)
        {
            driv+=xs[i]*pow(ans,i-1)*i;
        }
        ans=ans-value/driv;
        value=poly(xs,ans);
    }
    return ans;

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

#include<stdio.h>
#include<math.h>
#include<vector>
using namespace std;
double poly(vector<double> xs, double x){
    /* 
    Evaluates polynomial with coefficients xs at point x. return xs[0] + xs[1] * x + xs[1] * x^2 + .... xs[n] * x^n 
    */
    double sum=0;
    int i;
    for (i=0;i<xs.size();i++)
    {
        sum+=xs[i]*pow(x,i);
    }
    return sum;
}

double find_zero(vector<double> xs){
    /*
    xs are coefficients of a polynomial. find_zero find x such that poly(x) = 0. find_zero returns only only zero point, even if there are many. 
    Moreover, find_zero only takes list xs having even number of coefficients and largest non zero coefficient as it guarantees a solution.
    >>> round(find_zero([1, 2]), 2) #f(x) = 1 + 2x 
    -0.5 
    >>> round(find_zero([-6, 11, -6, 1]), 2) # (x - 1) * (x - 2) * (x - 3) = -6 + 11x - 6x^2 + x^3 
    1.0
    */


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
