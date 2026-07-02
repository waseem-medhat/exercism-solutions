(ns beer-song
  (:require [clojure.string :as cstr]))

(defn count->string [num]
  (if (zero? num) "no more" (str num)))

(defn plural-s [num]
  (if (= num 1) "" "s"))

(defn phrase-1 [num]
  (let [num-cur (count->string num)
        s-cur (plural-s num)]
    (format "%s bottle%s of beer on the wall, %s bottle%s of beer.\n"
            (cstr/capitalize num-cur)
            s-cur
            num-cur
            s-cur)))

(defn phrase-2 [num]
  (if (pos? num)
    (format "Take %s down and pass it around, %s bottle%s of beer on the wall.\n"
            (if (= 1 num) "it" "one")
            (count->string (dec num))
            (plural-s (dec num)))
    "Go to the store and buy some more, 99 bottles of beer on the wall.\n"))

(defn verse
  "Returns the nth verse of the song."
  [num]
  (str (phrase-1 num) (phrase-2 num)))

(defn sing
  "Given a start and an optional end, returns all verses in this interval. If
  end is not given, the whole song from start is sung."
  ([start] (sing start 0))
  ([start end]
   (->> (range end (inc start))
        reverse
        (map verse)
        (cstr/join "\n"))))
