open Base

let is_pangram txt =
  let rec aux chars letter_set =
    match chars with
    | [] -> Set.length letter_set = 26
    | c :: chars' when Char.is_alpha c ->
        aux chars' (Set.add letter_set (Char.lowercase c))
    | _ :: chars' -> aux chars' letter_set
  in
  aux (String.to_list txt) (Set.empty (module Char))
