(ns sublist)

(defn classify [list1 list2] ;; <- arglist goes here
  (let [sublist (fn [lst1 lst2]
                  (some #(= % lst1)
                        (partition (count lst1) 1 lst2)))]
    (cond
      (= list1 list2) :equal
      (sublist list1 list2) :sublist
      (sublist list2 list1) :superlist
      :else                 :unequal)))
