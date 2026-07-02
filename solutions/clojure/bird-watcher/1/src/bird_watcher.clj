(ns bird-watcher)

(def last-week 
  [0 2 5 3 7 8 4])

(defn today [birds]
  (last birds))

(defn inc-bird [birds]
  (let [last-index (dec (count birds))
        new-today (inc (last birds))]
    (assoc birds last-index new-today)))

(defn day-without-birds? [birds]
  (boolean (some zero? birds)))

(defn n-days-count [birds n]
  (apply + (take n birds)))

(defn busy-days [birds]
  (count (filter #(>= % 5) birds)))

(defn odd-week? [birds]
  (let [alternating (interleave (repeat 0) (repeat 1))
        odd1 (take 7 alternating)
        odd2 (rest (take 8 alternating))]
    (or (= birds odd1)
        (= birds odd2))))
