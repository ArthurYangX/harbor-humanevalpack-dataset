# Context

```python
def words_string(s):
    """
    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
    For example:
    words_string("Hi, my name is John") == ["Hi", "my", "name", "is", "John"]
    words_string("One, two, three, four, five, six") == ["One", "two", "three", "four", "five", "six"]
    """
    if not s:
        return []

    s_list = []

    for letter in s:
        if letter == ',':
            s_list.append(',')
        else:
            s_list.append(letter)

    s_list = "".join(s_list)
    return s_list.split()
```

# Instruction

Fix bugs in words_string.

# Prompt


def words_string(s):
    """
    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
    For example:
    words_string("Hi, my name is John") == ["Hi", "my", "name", "is", "John"]
    words_string("One, two, three, four, five, six") == ["One", "two", "three", "four", "five", "six"]
    """


# Instructions

Implement your solution in `solution/solution.py`.
Ensure your submission is self-contained and compiles/runs correctly.

```
