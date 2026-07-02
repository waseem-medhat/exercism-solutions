defmodule Anagram do
  @doc """
  Returns all candidates that are anagrams of, but not equal to, 'base'.
  """
  def frequencies(str),
    do: str |> String.downcase() |> String.graphemes() |> Enum.frequencies()

  @spec match(String.t(), [String.t()]) :: [String.t()]
  def match(base, candidates) do
    freq_base = frequencies(base)

    Enum.filter(candidates, fn candidate ->
      String.downcase(candidate) != String.downcase(base) and
        frequencies(candidate) == freq_base
    end)
  end
end

