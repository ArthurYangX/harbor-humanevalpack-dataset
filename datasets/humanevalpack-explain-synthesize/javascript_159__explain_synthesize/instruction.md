# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  You're a hungry rabbit, and you already have eaten a certain number of carrots,
  but now you need to eat more carrots to complete the day's meals.
  you should return an array of [ total number of eaten carrots after your meals,
                                  the number of carrots left after your meals ]
  if there are not enough remaining carrots, you will eat all remaining carrots, but will still be hungry.
  
  Example:
  * eat(5, 6, 10) -> [11, 4]
  * eat(4, 8, 9) -> [12, 1]
  * eat(1, 10, 10) -> [11, 0]
  * eat(2, 11, 5) -> [7, 0]
  
  Variables:
  @number : integer
      the number of carrots that you have eaten.
  @need : integer
      the number of carrots that you need to eat.
  @remaining : integer
      the number of remaining carrots thet exist in stock
  
  Constrain:
  * 0 <= number <= 1000
  * 0 <= need <= 1000
  * 0 <= remaining <= 1000

  Have fun :)
  */
const eat = (number, need, remaining) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
