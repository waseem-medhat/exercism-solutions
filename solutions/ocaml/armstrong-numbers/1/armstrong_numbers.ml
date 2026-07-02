let validate candidate =
  let rec get_digits acc n =
  match n with
    | n when n / 10 = 0 -> n :: acc
    | n -> get_digits ((n mod 10) :: acc) (n / 10)
  in
  let rec pow n p =
    int_of_float (Float.pow (float_of_int n) (float_of_int p))
  in
    let digits = get_digits [] candidate in
    let n_digits = List.length digits in
    let sum = digits
      |> List.map (fun x -> pow x n_digits)
      |> List.fold_left (+) 0 in
    sum = candidate