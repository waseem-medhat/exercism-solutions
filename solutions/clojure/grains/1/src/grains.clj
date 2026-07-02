(ns grains)

(defn square [n]
  (nth (iterate #(*' 2 %) 1) (-' n 1)))

(defn total []
  (->> (range 1 65)
    (map square)
    (apply +')))
