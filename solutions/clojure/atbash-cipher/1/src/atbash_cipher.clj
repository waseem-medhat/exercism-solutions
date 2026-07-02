(ns atbash-cipher
  (:require [clojure.string :as cstr]))

(defn clean
  "Converts a string to lowercase and removes whitespaces."
  [s]
  (-> s
    cstr/lower-case
    (cstr/replace #"[^\w\d]" "")))

(defn encode-letter
  "Encodes alphanumeric letters. Assumes to be used with cleaned strings."
  [l]
  (let [alphabet "abcdefghijklmnopqrstuvwxyz"
        rev-alphabet (reverse alphabet)]
    (if-let [l-index (cstr/index-of alphabet l)]
      (nth rev-alphabet l-index)
      l)))

(defn encode [s]
  (->> s
    clean
    (map encode-letter)
    (partition 5 5 nil)
    (interleave (repeat " "))
    flatten
    (apply str)
    cstr/trim))

