let egg_count number =
  let rec loop number acc =
    match number with
    | 0 -> acc
    | _ -> loop (number / 2) (acc + (number mod 2))
  in
  loop number 0
