(ns anagram
  (:require [clojure.string :as s]))

(def lowercase-freqs (comp frequencies s/lower-case))

(defn anagrams-for [word prospect-list]
  (let [word-freqs (lowercase-freqs word)
        not-self (filter #(not= (s/lower-case %)
                                (s/lower-case word))
                         prospect-list)]
    (filter #(= (lowercase-freqs %) word-freqs)
            not-self)))