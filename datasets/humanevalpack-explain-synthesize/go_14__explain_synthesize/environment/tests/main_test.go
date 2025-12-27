package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
)


// Prompt
// Return list of all prefixes from shortest to longest of the input string
// >>> AllPrefixes('abc')
// ['a', 'ab', 'abc']
func AllPrefixes(str string) []string{
// Solution
prefixes := make([]string, 0, len(str))
	for i := 0; i < len(str); i++ {
		prefixes = append(prefixes, str[:i+1])
	}
	return prefixes
}

// Test Code
func TestAllPrefixes(t *testing.T) {
    assert := assert.New(t)
    assert.Equal([]string{}, AllPrefixes(""))
    assert.Equal([]string{"a", "as", "asd", "asdf", "asdfg", "asdfgh"}, AllPrefixes("asdfgh"))
    assert.Equal([]string{"W", "WW", "WWW"}, AllPrefixes("WWW"))
}