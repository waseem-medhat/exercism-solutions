let rec length = function 
  | [] -> 0
  | _ :: t -> 1 + (length t)

let rec map ~f = function
  | [] -> []
  | h :: t -> (f h) :: (map ~f t)

let rec filter ~f = function
  | [] -> []
  | h :: t when f h -> h :: (filter ~f t)
  | _ :: t -> (filter ~f t)

let rec fold ~init ~f = function
  | [] -> init
  | h :: t -> fold ~init:(f init h) ~f t

let rec reverse l =
  fold ~f:(fun acc el -> el :: acc) ~init:[] l

let append a b =
  match a, b with
  | [], [] -> []
  | [], b -> b
  | a, [] -> a
  | _ ,_ ->
    let result = reverse a in
    let result = fold ~f:(fun acc el -> el :: acc) ~init:result b in
    reverse result

let concat list =
  let rec loop list acc =
    match list with
    | [] -> acc
    | sublist :: t -> loop t (append acc sublist)
  in
    loop list []