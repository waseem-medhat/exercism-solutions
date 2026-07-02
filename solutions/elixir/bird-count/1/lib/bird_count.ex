defmodule BirdCount do
  def today(list) do
    case list do
      [] -> nil
      [count | _] -> count
    end
  end

  def increment_day_count(list) do
    case list do
      [] -> [1]
      [count | rest] -> [count + 1 | rest]
    end
  end

  def has_day_without_birds?(list) do
    case list do
      [] -> false
      [0 | _] -> true
      [count | rest] -> has_day_without_birds?(rest)
    end
  end

  def total(list) do
    case list do
      [] -> 0
      [count | rest] -> count + total(rest)
    end
  end

  def busy_days(list) do
    case list do
      [] -> 0
      [count | rest] when count >= 5 -> 1 + busy_days(rest)
      [_ | rest] -> busy_days(rest)
    end
  end
end
