open Base

let is_opening = function
  | '[' | '{' | '(' -> true
  | _ -> false

let is_closing = function
  | ']' | '}' | ')' -> true
  | _ -> false

let is_bracket c = is_opening c || is_closing c

let matches opening closing =
  match (opening, closing) with
  | '[', ']' | '{', '}' | '(', ')' -> true
  | _ -> false

let are_balanced str =
  let rec loop chars stack =
    match (chars, stack) with
    | [], [] -> true
    | [], _ -> false
    | c :: chars', stack when not (is_bracket c) -> loop chars' stack
    | c :: chars', stack when is_opening c -> loop chars' (c :: stack)
    | c :: chars', top :: stack' when matches top c -> loop chars' stack'
    | _ -> false
  in
  loop (String.to_list str) []
