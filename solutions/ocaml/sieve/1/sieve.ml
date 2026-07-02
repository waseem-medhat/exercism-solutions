let primes n =
  let rec aux candidates primes =
    match candidates with
    | [] -> primes
    | c :: candidates' ->
        aux (List.filter (fun num -> num mod c <> 0) candidates') (c :: primes)
  in
  aux (List.init (n - 1) (fun i -> i + 2)) []