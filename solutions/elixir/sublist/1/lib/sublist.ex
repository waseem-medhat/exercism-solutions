defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare(a, b) do
    len_a = Enum.count(a)
    len_b = Enum.count(b)

    cond do
      a === b ->
        :equal

      a === [] ->
        :sublist

      b === [] ->
        :superlist

      len_a > len_b ->
        a
        |> Enum.chunk_every(len_b, 1)
        |> Enum.find_value(:unequal, fn chunk -> if chunk === b, do: :superlist end)

      len_b > len_a ->
        b
        |> Enum.chunk_every(len_a, 1)
        |> Enum.find_value(:unequal, fn chunk -> if chunk === a, do: :sublist end)

      true ->
        :unequal
    end
  end
end
