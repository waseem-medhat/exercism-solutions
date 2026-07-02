defmodule Username do
  def sanitize(username) do
    sanitize(username, ~c"")
  end

  defguardp allowed?(c) when (c >= ?a and c <= ?z) or c == ?_

  defp sanitize(username, sanitized) do
    case username do
      [] -> Enum.reverse(sanitized)
      [c | rest] when allowed?(c) -> sanitize(rest, [c | sanitized])
      [?ä | rest] -> sanitize(rest, [?e | [?a | sanitized]])
      [?ö | rest] -> sanitize(rest, [?e | [?o | sanitized]])
      [?ü | rest] -> sanitize(rest, [?e | [?u | sanitized]])
      [?ß | rest] -> sanitize(rest, [?s | [?s | sanitized]])
      [_ | rest] -> sanitize(rest, sanitized)
    end
  end
end
