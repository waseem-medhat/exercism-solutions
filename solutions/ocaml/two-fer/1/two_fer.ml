let two_fer = function
  | None -> "One for you, one for me."
  | Some(name) -> Printf.sprintf "One for %s, one for me." name