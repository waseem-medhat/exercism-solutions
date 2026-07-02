(ns grains)

(defn square [n]
  (.pow (biginteger 2) (dec n)))

(defn total []
  (->> (range 1 65)
    (map square)
    (apply +')))
