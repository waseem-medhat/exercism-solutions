let score word =
  let score_letter = function
    | 'D' | 'G' -> 2
    | 'B' | 'C' | 'M' | 'P' -> 3
    | 'F' | 'H' | 'V' | 'W' | 'Y' -> 4
    | 'K' -> 5
    | 'J' | 'X' -> 8
    | 'Q' | 'Z' -> 10
    | _ -> 1
  in
  String.fold_left
    (fun acc c -> acc + score_letter c)
    0
    (String.uppercase_ascii word)