let raindrop n = 
  n 
  |> (fun n -> if n mod 3 = 0 then "Pling" else "") 
  |> (fun s -> if n mod 5 = 0 then s ^ "Plang" else s) 
  |> (fun s -> if n mod 7 = 0 then s ^ "Plong" else s)
  |> (fun s -> if s = "" then string_of_int n else s)