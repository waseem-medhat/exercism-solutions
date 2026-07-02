let acronym str =
  let rec loop str acc is_initial =
    if str = "" then acc else
      let str_hd = String.sub str 0 1 in
      let str_tl = String.sub str 1 ((String.length str) - 1) in
      match str_hd with
      | " " | "-" | "_"   -> loop str_tl acc true
      | _ when is_initial -> loop str_tl (acc ^ (String.uppercase_ascii str_hd)) false
      | _                 -> loop str_tl acc false
  in
    loop str "" true