open Base

let response_for text =
  let is_question text = text |> String.strip |> String.is_suffix ~suffix:"?" in
  let is_shout text =
    String.exists text ~f:Char.is_uppercase
    && not (String.exists text ~f:Char.is_lowercase)
  in
  match (is_question text, is_shout text) with
  | true, false -> "Sure."
  | false, true -> "Whoa, chill out!"
  | true, true -> "Calm down, I know what I'm doing!"
  | _, _ when String.is_empty (String.strip text) -> "Fine. Be that way!"
  | _, _ -> "Whatever."
