type nucleotide = A | C | G | T

let hamming_distance s1 s2 =
  let rec distance_rec strands acc =
    match strands with
      | [] -> acc
      | (n1, n2) :: tl when n1 <> n2 -> distance_rec tl acc+1
      | _ :: tl -> distance_rec tl acc
  in
    if List.length s1 <> List.length s2 then
      Error "strands must be of equal length"
    else
      Ok (distance_rec (List.combine s1 s2) 0);