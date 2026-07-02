(ns triangle)

(defn is-valid?
  [side1 side2 side3]
  (let [sides (sort [side1 side2 side3])]
  (and (every? pos? sides)
       (>= (+ (first sides)
              (second sides))
           (nth sides 2)))))

(defn equilateral? [& sides]
  (and (apply is-valid? sides)
       (apply = sides)))

(defn isosceles? [& sides]
  (and (apply is-valid? sides)
       (let [[side1 side2 side3] (sort sides)]
         (or (= side1 side2)
             (= side2 side3)))))

(defn scalene? [& sides]
  (and (apply is-valid? sides)
       (not (apply isosceles? sides))))
