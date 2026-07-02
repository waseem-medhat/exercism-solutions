defmodule HighScore do
  @init_score 0

  def new(), do: %{}
  def add_player(scores, name, score \\ @init_score), do: Map.put(scores, name, score)
  def remove_player(scores, name), do: Map.delete(scores, name)
  def reset_score(scores, name), do: Map.put(scores, name, @init_score)

  def update_score(scores, name, score) do
    case scores do
      %{^name => current_score} -> Map.put(scores, name, score + current_score)
      _ -> add_player(scores, name, score)
    end
  end

  def get_players(scores), do: Map.keys(scores)
end
