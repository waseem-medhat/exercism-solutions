(ns collatz-conjecture)

(defn next-num
  [n]
  (cond
    (<= n 0)  (throw (Exception. "Not a valid input"))
    (even? n) (/ n 2)
    :else     (+ 1 (* 3 n))))

(defn collatz [num]
  (loop [current-num num
         steps 0]
    (if (= current-num 1)
      steps
      (recur (next-num current-num)
             (+ 1 steps)))))
