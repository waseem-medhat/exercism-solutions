(ns cars-assemble)

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (cond
    (== speed 0)   0.0
    (<= 1 speed 4) (* 221.0 speed)
    (<= 5 speed 8) (* 0.9 221.0 speed)
    (== speed 9)   (* 0.8 221.0 speed)
    :else          1701.7))

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (-> speed
    production-rate
    (/ 60)
    int))
