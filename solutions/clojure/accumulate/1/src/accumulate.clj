(ns accumulate)

(defn accumulate [op lst]
  (reduce (fn [new-lst next-el]
            (conj new-lst (op next-el)))
          []
          lst))
