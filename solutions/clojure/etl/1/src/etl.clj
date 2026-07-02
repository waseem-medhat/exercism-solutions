(ns etl)

(defn transform-item
  [[k v]]
  (zipmap (map clojure.string/lower-case v)
          (repeat (count v) k)))

(defn transform
  [source]
  (apply conj (map transform-item source)))
