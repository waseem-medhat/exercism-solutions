(ns rna-transcription)

(defn to-rna [dna]
  (assert (not (re-find #"[^GCTA]" dna)))
  (let [nucleotide-map {\G "C"
                        \C "G"
                        \T "A"
                        \A "U"}]
    (->> dna
      (map nucleotide-map)
      (apply str))))
