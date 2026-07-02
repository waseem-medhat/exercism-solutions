(ns rotational-cipher
    (:require [clojure.string :as cstr]))

(def caps "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
(def smalls "abcdefghijklmnopqrstuvwxyz")

(defn rotate-from-alphabet
  "Rotate a letter from a given alphabet."
  [letter alphabet dist]
  (-> (cstr/index-of alphabet letter)
      (+ dist)
      (rem 26)
      (#(nth alphabet %))))

(defn rotate-letter
  [l dist]
  (cond
    (cstr/includes? caps l)   (rotate-from-alphabet l caps dist)
    (cstr/includes? smalls l) (rotate-from-alphabet l smalls dist)
    :else                     l))

(defn rotate
  [s dist]
  (->> (cstr/split s #"")
       (map #(rotate-letter % dist))
       (apply str)))

