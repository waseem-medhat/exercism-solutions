type dna = [ `A | `C | `G | `T ]
type rna = [ `A | `C | `G | `U ]

let to_rna strand =
    let convert (nuc: dna): rna =
      match nuc with
      | `A -> `U 
      | `T -> `A 
      | `C -> `G
      | `G -> `C
    in
      List.map convert strand