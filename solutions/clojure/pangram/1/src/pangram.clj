(ns pangram
  (:require [clojure.string :as s]))

(defn pangram? [string]
  (-> string
    s/lower-case
    (s/replace #"[\W\d]" "")
    set
    count
    (>= 26)))
