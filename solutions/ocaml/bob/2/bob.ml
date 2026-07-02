open Base

let response_for text =
  let text = String.strip text in
  let is_question = String.is_suffix text ~suffix:"?" in
  let is_shout =
    String.exists text ~f:Char.is_uppercase
    && not (String.exists text ~f:Char.is_lowercase)
  in
  match (is_question, is_shout) with
  | true, false -> "Sure."
  | false, true -> "Whoa, chill out!"
  | true, true -> "Calm down, I know what I'm doing!"
  | _, _ when String.is_empty text -> "Fine. Be that way!"
  | _, _ -> "Whatever."
