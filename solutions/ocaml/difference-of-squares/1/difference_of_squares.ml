open Base

let range i = List.init i ~f:(fun n -> (n + 1))
let square n = n ** 2
let sum n_list = List.fold n_list ~init:zero ~f:(+)

let square_of_sum i =
    (range i) |> sum |> square

let sum_of_squares i = 
    (range i) |> List.map ~f:square |> sum

let difference_of_squares i =
    (square_of_sum i) - (sum_of_squares i)
