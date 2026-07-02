(ns accumulate)

(defn accumulate [op lst]
  (if (empty? lst)
    lst
    (cons (op (first lst))
          (accumulate op (rest lst)))))
