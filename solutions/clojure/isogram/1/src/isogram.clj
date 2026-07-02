(ns isogram
    (:require [clojure.string :as cstr]))

(defn isogram? [word]
  (->> (cstr/replace word #"[\s-]" "")
    cstr/lower-case
    frequencies
    vals
    (every? #(= 1 %))))

