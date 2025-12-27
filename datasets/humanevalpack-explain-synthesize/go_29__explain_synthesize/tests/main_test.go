package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
)


// Prompt
// Filter an input list of strings only for ones that start with a given prefix.
// >>> FilterByPrefix([], 'a')
// []
// >>> FilterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
// ['abc', 'array']
func FilterByPrefix(strings []string,prefix string) []string {
// Solution
if len(strings) == 0 {
        return []string{}
    }
    res := make([]string, 0, len(strings))
	for _, s := range strings {
		if s[:len(prefix)] == prefix {
			res = append(res, s)
		}
	}
	return res
}

// Test Code
func TestFilterByPrefix(t *testing.T) {
    assert := assert.New(t)
    assert.Equal([]string{}, FilterByPrefix([]string{}, "john"))
    assert.Equal([]string{"xxx", "xxxAAA", "xxx"}, FilterByPrefix([]string{"xxx", "asd", "xxy", "john doe", "xxxAAA", "xxx"}, "xxx"))
}