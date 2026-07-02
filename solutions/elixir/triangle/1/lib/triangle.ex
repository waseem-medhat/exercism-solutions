defmodule Triangle do
  @type kind :: :equilateral | :isosceles | :scalene

  defguardp inequality?(a, b, c) when a + b >= c and a + c >= b and b + c >= a
  defguardp positive_lengths?(a, b, c) when a > 0 and b > 0 and c > 0

  @doc """
  Return the kind of triangle of a triangle with 'a', 'b' and 'c' as lengths.
  """
  @spec kind(number, number, number) :: {:ok, kind} | {:error, String.t()}
  def kind(a, b, c) do
    case {a, b, c} do
      {a, b, c} when not positive_lengths?(a, b, c) ->
        {:error, "all side lengths must be positive"}

      {a, b, c} when not inequality?(a, b, c) ->
        {:error, "side lengths violate triangle inequality"}

      {a, a, a} ->
        {:ok, :equilateral}

      {a, _, a} ->
        {:ok, :isosceles}

      {a, a, _} ->
        {:ok, :isosceles}

      {_, b, b} ->
        {:ok, :isosceles}

      _ ->
        {:ok, :scalene}
    end
  end
end
