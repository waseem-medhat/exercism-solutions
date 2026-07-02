defmodule FlattenArray do
  @doc """
    Accept a list and return the list flattened without nil values.

    ## Examples

      iex> FlattenArray.flatten([1, [2], 3, nil])
      [1, 2, 3]

      iex> FlattenArray.flatten([nil, nil])
      []

  """

  @spec flatten(list) :: list
  def flatten(list), do: flatten(list, [])

  defp flatten(list, acc) do
    case list do
      [] -> Enum.reverse(acc)
      [[] | tail] -> flatten(tail, acc)
      [[_ | _] = head | tail] -> flatten(head ++ tail, acc)
      [nil | tail] -> flatten(tail, acc)
      [head | tail] -> flatten(tail, [head | acc])
    end
  end
end
