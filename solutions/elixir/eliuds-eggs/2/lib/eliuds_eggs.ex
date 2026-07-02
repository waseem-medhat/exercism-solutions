defmodule EliudsEggs do
  @doc """
  Given the number, count the number of eggs.
  """
  @spec egg_count(number :: integer()) :: non_neg_integer()
  def egg_count(number) do
    case number do
      0 -> 0
      n -> rem(n, 2) + egg_count(div(n, 2))
    end
  end
end
