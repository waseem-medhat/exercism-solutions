defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare([], []), do: :equal
  def compare(_a, []), do: :superlist
  def compare([], _b), do: :sublist
  def compare(a, a), do: :equal

  def compare(a, b) do
    len_a = Enum.count(a)
    len_b = Enum.count(b)

    cond do
      len_a > len_b and b in Enum.chunk_every(a, len_b, 1) -> :superlist
      len_b > len_a and a in Enum.chunk_every(b, len_a, 1) -> :sublist
      true -> :unequal
    end
  end
end
