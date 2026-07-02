let score (x: float) (y: float): int =
  let dist = 
    (Float.pow x 2.0)
    |> Float.add (Float.pow y 2.0)
    |> Float.sqrt
  in
  match () with
  | _ when dist > 10.0 -> 0
  | _ when dist > 5.0 -> 1
  | _ when dist > 1.0 -> 5
  | _ -> 10