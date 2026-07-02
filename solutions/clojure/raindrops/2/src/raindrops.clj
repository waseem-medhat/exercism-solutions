(ns raindrops)

(defn make-sound
  [num factor sound]
  (when (zero? (mod num factor))
    sound))

(defn convert [num]
  (let [pling-plang-plong (str (make-sound num 3 "Pling")
                               (make-sound num 5 "Plang")
                               (make-sound num 7 "Plong"))]
    (if (= pling-plang-plong "")
      (str num)
      pling-plang-plong)))