defmodule ResistorColorTrio do
  @doc """
  Calculate the resistance value in ohms from resistor colors
  """
  @spec label(colors :: [atom]) :: {number, :ohms | :kiloohms | :megaohms | :gigaohms}
  def label([color_1 | [color_2 | [color_3 | _]]]) do
    code_1 = code(color_1)
    code_2 = code(color_2)
    code_3 = code(color_3)

    case code_2 do
      0 ->
        {unit, rem_zeros} = to_unit(code_3 + 1)
        {shift(code_1, rem_zeros), unit}

      _ ->
        {unit, rem_zeros} = to_unit(code_3)
        {shift(code_1, rem_zeros + 1) + shift(code_2, rem_zeros), unit}
    end
  end

  defp shift(n, num_zeros), do: n * 10 ** num_zeros

  defp to_unit(n_zeros) do
    cond do
      n_zeros >= 9 -> {:gigaohms, rem(n_zeros, 9)}
      n_zeros >= 6 -> {:megaohms, rem(n_zeros, 6)}
      n_zeros >= 3 -> {:kiloohms, rem(n_zeros, 3)}
      true -> {:ohms, n_zeros}
    end
  end

  defp code(color) do
    case color do
      :black -> 0
      :brown -> 1
      :red -> 2
      :orange -> 3
      :yellow -> 4
      :green -> 5
      :blue -> 6
      :violet -> 7
      :grey -> 8
      :white -> 9
    end
  end
end
