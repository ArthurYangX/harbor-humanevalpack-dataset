# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// There are eight planets in our solar system: the closerst to the Sun
// is Mercury, the next one is Venus, then Earth, Mars, Jupiter, Saturn,
// Uranus, Neptune.
// Write a function that takes two planet names as strings planet1 and planet2.
// The function should return a tuple containing all planets whose orbits are
// located between the orbit of planet1 and the orbit of planet2, sorted by
// the proximity to the sun.
// The function should return an empty tuple if planet1 or planet2
// are not correct planet names.
// Examples
// Bf("Jupiter", "Neptune") ==> ("Saturn", "Uranus")
// Bf("Earth", "Mercury") ==> ("Venus")
// Bf("Mercury", "Uranus") ==> ("Venus", "Earth", "Mars", "Jupiter", "Saturn")
func Bf(planet1, planet2 string) []string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
