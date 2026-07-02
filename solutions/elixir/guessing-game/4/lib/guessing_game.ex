defmodule GuessingGame do
  def compare(answer, guess \\ :no_guess)
  
  def compare(answer, answer) do
    "Correct"
  end

 def compare(_answer, :no_guess) do
    "Make a guess"
  end

  def compare(answer, guess) when abs(guess - answer) == 1  do
    "So close"
  end 

  def compare(answer, guess) when guess > answer do
    "Too high"
  end

  def compare(answer, guess) when guess < answer do
    "Too low"
  end
end
