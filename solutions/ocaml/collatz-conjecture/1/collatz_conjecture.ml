let collatz_conjecture n =
  let rec loop n count =
    match n with
    | 1 -> count
    | n when n mod 2 = 0 -> loop (n / 2) (count + 1)
    | n -> loop ((3 * n) + 1) (count + 1)
  in
  match n with
  | n when n <= 0 -> Error "Only positive integers are allowed"
  | n -> Ok (loop n 0)
