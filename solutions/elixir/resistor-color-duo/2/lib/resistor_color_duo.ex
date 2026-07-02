defmodule ResistorColorDuo do
  @doc """
  Calculate a resistance value from two colors
  """
  @spec value(colors :: [atom]) :: integer
  def value([c1 | [c2 | _]]) do
    str = "#{code(c1)}#{code(c2)}"
    {parsed, _} = Integer.parse(str)
    parsed
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
