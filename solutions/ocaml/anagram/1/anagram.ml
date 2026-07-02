open Base

let anagrams word word_lst =
    let empty = Map.empty (module Char) in
    let count_letters word =
        String.fold word ~init:empty ~f:(fun counts ch ->
            let letter = (Char.uppercase ch) in
            match Map.find counts letter with
            | Some count -> Map.set counts ~key:letter ~data:(count + 1)
            | None       -> Map.set counts ~key:letter ~data:0
        )
    in
    let is_anagram word1 word2 = 
        String.(<>) (String.uppercase word1) (String.uppercase word2) &&
            Map.equal equal_int (count_letters word1) (count_letters word2)
    in
        List.filter ~f:(fun other_word -> is_anagram word other_word) word_lst

