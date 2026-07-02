(ns nucleotide-count)

(defn count-of-nucleotide-in-strand [nucleotide strand]
  (when (re-find #"[^ACGT]" strand)
    (throw (Exception. "Not a valid strand.")))
  (if (contains? #{\A \C \G \T} nucleotide)
    (count (filter #{nucleotide} (seq strand)))
    (throw "Not a valid nucleotide")))

(defn nucleotide-counts [strand]
    (reduce (fn [counts nucleotide]
              (assoc counts
                     nucleotide
                     (count-of-nucleotide-in-strand nucleotide strand)))
            {}
            #{\A \C \G \T}))
