(ns elyses-destructured-enchantments)

(defn first-card
  "Returns the first card from deck."
  [deck]
  (first deck))

(defn second-card
  "Returns the second card from deck."
  [[first-card & rest-deck]]
  (first rest-deck))

(defn swap-top-two-cards
  "Returns the deck with first two items reversed."
  [[first-card second-card & rest-deck]]
  (into [second-card first-card] rest-deck))

(defn discard-top-card
  "Returns a sequence containing the first card and
   a sequence of the remaining cards in the deck."
  [[first-card & rest-deck]]
  [first-card rest-deck])

(def face-cards
  ["jack" "queen" "king"])

(defn insert-face-cards
  "Returns the deck with face cards between its head and tail."
  [deck]
  (concat (take 1 deck)
          face-cards
          (rest deck)))
