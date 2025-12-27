# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Input to this function is a string representing musical notes in a special ASCII format.
// Your task is to parse this string and return list of integers corresponding to how many beats does each
// not last.
// 
// Here is a legend:
// 'o' - whole note, lasts four beats
// 'o|' - half note, lasts two beats
// '.|' - quater note, lasts one beat
// 
// >>> ParseMusic('o o| .| o| o| .| .| .| .| o o')
// [4, 2, 1, 2, 2, 1, 1, 1, 1, 4, 4]
func ParseMusic(music_string string) []int{
    note_map := map[string]int{"o": 4, "o|": 2, ".|": 1}
	split := strings.Split(music_string, " ")
	result := make([]int, 0)
	for _, x := range split {
		if i, ok := note_map[x]; ok {
			result = append(result, i)
		}
	}
	return result
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


// Input to this function is a string representing musical notes in a special ASCII format.
// Your task is to parse this string and return list of integers corresponding to how many beats does each
// not last.
// 
// Here is a legend:
// 'o' - whole note, lasts four beats
// 'o|' - half note, lasts two beats
// '.|' - quater note, lasts one beat
// 
// >>> ParseMusic('o o| .| o| o| .| .| .| .| o o')
// [4, 2, 1, 2, 2, 1, 1, 1, 1, 4, 4]
func ParseMusic(music_string string) []int{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
