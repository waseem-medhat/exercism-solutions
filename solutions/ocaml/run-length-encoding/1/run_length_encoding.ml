open Base

let encode str =
  let append enc count char =
    match count with
    | 1 -> enc ^ Char.to_string char
    | count -> enc ^ Int.to_string count ^ Char.to_string char
  in
  let rec loop chars ~ch ~count ~enc =
    match chars with
    | [] -> append enc count ch
    | first :: rest when Char.( = ) first ch ->
        loop rest ~ch ~count:(count + 1) ~enc
    | first :: rest -> loop rest ~ch:first ~count:1 ~enc:(append enc count ch)
  in
  match String.to_list str with
  | [] -> ""
  | first :: rest -> loop rest ~ch:first ~count:1 ~enc:""

let decode str =
  let append decoded count char =
    match count with
    | "" -> decoded ^ String.of_char char
    | count -> decoded ^ String.make (Int.of_string count) char
  in
  let rec loop chars ~count ~decoded =
    match chars with
    | [] -> decoded
    | digit :: rest when Char.is_digit digit ->
        loop rest ~count:(count ^ Char.to_string digit) ~decoded
    | letter :: rest ->
        loop rest ~count:"" ~decoded:(append decoded count letter)
  in
  loop (String.to_list str) ~count:"" ~decoded:""
