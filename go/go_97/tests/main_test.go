package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
    "math"
)


// Prompt
// Complete the function that takes two integers and returns
// the product of their unit digits.
// Assume the input is always valid.
// Examples:
// Multiply(148, 412) should return 16.
// Multiply(19, 28) should return 72.
// Multiply(2020, 1851) should return 0.
// Multiply(14,-15) should return 20.
func Multiply(a, b int) int {
// Solution
return int(math.Abs(float64(a%10)) * math.Abs(float64(b%10)))
}

// Test Code
func TestMultiply(t *testing.T) {
    assert := assert.New(t)
    assert.Equal(16, Multiply(148, 412))
    assert.Equal(72, Multiply(19, 28))
    assert.Equal(0, Multiply(2020, 1851))
    assert.Equal(20, Multiply(14, -15))
    assert.Equal(42, Multiply(76, 67))
    assert.Equal(49, Multiply(17, 27))
    assert.Equal(0, Multiply(0, 1))
    assert.Equal(0, Multiply(0, 0))
}
