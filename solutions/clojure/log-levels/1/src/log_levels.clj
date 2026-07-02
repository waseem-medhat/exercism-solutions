(ns log-levels
  (:require [clojure.string :as str]))

(defn extract-pieces
  [s]
  (re-find #"\[(INFO|WARNING|ERROR)\]: (.+)" s))

(defn message
  "Takes a string representing a log line
   and returns its message with whitespace trimmed."
  [s]
  (str/trim ((extract-pieces s) 2)))

(defn log-level
  "Takes a string representing a log line
   and returns its level in lower-case."
  [s]
  (str/lower-case ((extract-pieces s) 1)))

(defn reformat
  "Takes a string representing a log line and formats it
   with the message first and the log level in parentheses."
  [s]
  (str (message s)
       " ("
       (log-level s)
       ")"))
