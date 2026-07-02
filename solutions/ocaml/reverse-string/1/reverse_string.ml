let reverse_string s =
    s 
    |> String.fold_left (fun acc b -> Seq.cons b acc) Seq.empty
    |> String.of_seq