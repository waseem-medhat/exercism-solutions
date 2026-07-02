(ns bob
  (:require [clojure.string :as s]))

(defn question?
  [s]
  (boolean (re-find #"\?$" s))) ; has "?" at the end

(defn yell?
  [s]
  (and (re-find #"[A-Z]+" s)    ; has any letter
       (= s (s/upper-case s)))) ; all letters are uppercase

(defn yell-question?
  [s]
  (and (question? s)
       (yell? s)))

(defn silence?
  [s]
  (boolean (re-find #"^\s*$" s))) ; whole string is zero or more whitespaces

(defn response-for [s]
  (let [s (clojure.string/trim s)]
      (cond
        (silence? s)       "Fine. Be that way!"
        (yell-question? s) "Calm down, I know what I'm doing!"
        (question? s)      "Sure."
        (yell? s)          "Whoa, chill out!"
        :else              "Whatever.")))
