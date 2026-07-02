(ns protein-translation)

(defn translate-codon [codon]
      (case codon
        "AUG"                     "Methionine"
        ("UUC" "UUU")             "Phenylalanine"
        ("UUA" "UUG")             "Leucine"
        ("UCU" "UCC" "UCA" "UCG") "Serine"
        ("UAU" "UAC")             "Tyrosine"
        ("UGU" "UGC")             "Cysteine"
        "UGG"                     "Tryptophan"
        ("UAA" "UAG" "UGA")       "STOP"))

(defn translate-rna [strand]
  (->> strand
    (partition 3)
    (map #(apply str %))
    (map translate-codon)
    (take-while #(not= % "STOP"))))
