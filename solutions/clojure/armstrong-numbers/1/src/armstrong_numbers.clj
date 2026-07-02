(ns armstrong-numbers)

(defn armstrong? [num]
  (let [digits-str (clojure.string/split (str (bigint num)) #"")
        n-digits (count digits-str)
        power-sum (apply +'
                         (map #(.pow (BigInteger. %) n-digits)
                              digits-str))]
    (= num power-sum)))
