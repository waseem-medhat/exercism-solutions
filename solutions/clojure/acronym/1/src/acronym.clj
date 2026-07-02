(ns acronym
  (:require [clojure.string :as s]))

(defn capitals
  "Given a word, get the first letter (as upper case)
  and other capital characters"
  [[first & rest]]
  (apply (partial str (s/upper-case first))
         (filter #(Character/isUpperCase %) rest)))

(defn acronymize-word
  "Acronymize a single word"
  [word]
  (cond 
    (s/blank? word)              ""
    (= word (s/upper-case word)) (subs word 0 1)
    :else                        (capitals word)))

(defn acronym
  "Converts phrase to its acronym."
  [phrase]
  (->> (s/split phrase #"\b")         ; split by 'word boundary'
    (filter #(re-matches #"^\w*$" %)) ; filter items with only letters
    (map acronymize-word)             ; get first char and capitals from each word
    (apply str)))                     ; concatenate
