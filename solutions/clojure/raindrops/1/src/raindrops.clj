(ns raindrops)

(defn convert [num]
  (let [pling-plang-plong (str (when (zero? (mod num 3))
                                 "Pling")
                               (when (zero? (mod num 5))
                                 "Plang")
                               (when (zero? (mod num 7))
                                 "Plong"))]
    (if (= pling-plang-plong "")
      (str num)
      pling-plang-plong)))
