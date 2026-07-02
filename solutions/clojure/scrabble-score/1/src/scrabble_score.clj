(ns scrabble-score
    (:require [clojure.string :as cstr]))

(defn score-letter
  [letter]
  (condp #(cstr/includes? %1 %2) (cstr/upper-case letter)
    "AEIOULNRST" 1
    "DG"         2
    "BCMP"       3
    "FHVWY"      4
    "K"          5
    "JX"         8
    "QZ"         10))


(defn score-word
  [word]
  (apply + (map score-letter word)))
