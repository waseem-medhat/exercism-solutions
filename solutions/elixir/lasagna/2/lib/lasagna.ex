defmodule Lasagna do
  def expected_minutes_in_oven(), do: 40

  def remaining_minutes_in_oven(mins), do: expected_minutes_in_oven() - mins

  def preparation_time_in_minutes(n_layers), do: 2 * n_layers

  def total_time_in_minutes(n_layers, mins_in_oven) do
    preparation_time_in_minutes(n_layers) + mins_in_oven
  end

  def alarm(), do: "Ding!"
end
