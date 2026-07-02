(ns squeaky-clean
  (:require [clojure.string :as str]))

(defn spaces-to-underscores
  [s]
  (str/replace s #" " "_"))

(defn clean-control-chars
  [s]
  (->> s
    (map #(if (Character/isISOControl %)
            "CTRL"
            %))
    (apply str)))

(defn camelize-case
  [s]
  (str/replace s 
               #"-\p{Ll}"
               #(str/upper-case (subs % 1))))

(defn remove-non-letters
  [s]
  (str/replace s
               #"[^\p{L}_]"
               ""))

(defn remove-greek
  [s]
  (str/replace s
               #"[α-ω]"
               ""))

(defn clean
  "Cleans identifier names"
  [s]
  (-> s
    spaces-to-underscores
    clean-control-chars
    camelize-case
    remove-non-letters
    remove-greek))
