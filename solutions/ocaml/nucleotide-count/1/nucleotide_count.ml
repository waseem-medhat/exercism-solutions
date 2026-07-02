open Base

let empty = Map.empty (module Char)
let valid_nucleotides = ['A'; 'C'; 'G'; 'T']
let is_valid nucleotide = List.mem valid_nucleotides nucleotide ~equal:equal_char
let tail s = String.sub s ~pos:1 ~len:(String.length s - 1)

let count_nucleotide strand nucleotide =
  let rec loop strand result =
    if String.length strand = 0 then result else
    match result, strand.[0] with
    | Ok count, c when equal_char c nucleotide -> loop (tail strand) (Ok (count + 1))
    | Ok _, c when is_valid c -> loop (tail strand) result
    | _, _ -> Error 'X'
  in
    if not (is_valid nucleotide) then Error 'X'
    else loop strand (Ok 0)

let count_nucleotides strand =
  let aux acc c =
    match acc, (count_nucleotide strand c) with
    | Ok counts_map, Ok 0 -> acc
    | Ok counts_map, Ok count -> Ok (Map.add_exn counts_map ~key:c ~data:count)
    | _, _ -> Error 'X'
  in
    List.fold valid_nucleotides ~f:aux ~init:(Ok empty)

