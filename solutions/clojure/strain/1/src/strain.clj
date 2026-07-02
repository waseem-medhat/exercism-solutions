(ns strain)

(defn retain [pred coll]
  (reduce 
   (fn [new-coll next-el]
     (if (pred next-el)
       (conj new-coll next-el)
       new-coll))
     []
     coll))

(defn discard [pred coll]
  (retain (complement pred) coll))
