# Problem Description

Write a Go function `func FindZero(xs []int) float64` to solve the following problem:
xs are coefficients of a polynomial.
FindZero find x such that Poly(x) = 0.
FindZero returns only only zero point, even if there are many.
Moreover, FindZero only takes list xs having even number of coefficients
and largest non zero coefficient as it guarantees
a solution.
>>> round(FindZero([1, 2]), 2) # f(x) = 1 + 2x
-0.5
>>> round(FindZero([-6, 11, -6, 1]), 2) # (x - 1) * (x - 2) * (x - 3) = -6 + 11x - 6x^2 + x^3
1.0

# Prompt

import (
    "math"
)

// Evaluates polynomial with coefficients xs at point x.
// return xs[0] + xs[1] * x + xs[1] * x^2 + .... xs[n] * x^n
func Poly(xs []int, x float64) float64{
    sum := 0.0
    for i, coeff := range xs {
        sum += float64(coeff) * math.Pow(x,float64(i))
    }
    return sum
}
// xs are coefficients of a polynomial.
// FindZero find x such that Poly(x) = 0.
// FindZero returns only only zero point, even if there are many.
// Moreover, FindZero only takes list xs having even number of coefficients
// and largest non zero coefficient as it guarantees
// a solution.
// >>> round(FindZero([1, 2]), 2) # f(x) = 1 + 2x
// -0.5
// >>> round(FindZero([-6, 11, -6, 1]), 2) # (x - 1) * (x - 2) * (x - 3) = -6 + 11x - 6x^2 + x^3
// 1.0
func FindZero(xs []int) float64 {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
