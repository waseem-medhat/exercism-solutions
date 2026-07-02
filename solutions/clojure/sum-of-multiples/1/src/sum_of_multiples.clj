(ns sum-of-multiples)

(defn multiples
  [num mult]
  (filter #(zero? (rem % mult))
          (range num)))

(defn sum-of-multiples
  [mults num]
  (->> (mapcat #(multiples num %) mults)
       (apply hash-set)
       (apply +)))
