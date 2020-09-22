(ns e68.core
  (:require [clojure.math.combinatorics :as comb]
            [clojure.set :as cset]
            )
  )

;(def ALL-NUMBERS #{1 2 3 4 5 6})
;(def GROUPS [[3 2 1] [5 1 0] [4 0 2]])

(def ALL-NUMBERS #{1 2 3 4 5 6 7 8 9 10})
(def GROUPS [[5 0 1] [6 1 2] [7 2 3] [8 3 4] [9 4 0]]) 

(defn make-rings [ring-size n]
  (->>
   (comb/combinations (range 1 (inc n)) ring-size)
   (map #(cons (last %) %))
   (map #(partition 2 1 %))
   )
  )

(defn select-from [v idxs]
  (mapv v idxs)
  )

(defn sums [arrangement]
  [
   (reduce + (select-from arrangement  (GROUPS 0))) 
   (reduce + (select-from arrangement  (GROUPS 1))) 
   (reduce + (select-from arrangement  (GROUPS 2))) 
   (reduce + (select-from arrangement  (GROUPS 3))) 
   (reduce + (select-from arrangement  (GROUPS 4))) 
   ]
  )

(defn all-equal? [v]
  (every? #(= % (first v)) v)
  )

(defn get-index-of-min [v]
  (first (apply min-key second (map-indexed vector v)))
  )

(defn to-string [arrangement]
  (let [groups (mapv #(mapv arrangement %) GROUPS)
        firsts (mapv first groups)
        idx (get-index-of-min firsts)
        groups-as-strings (mapv #(apply str %) groups)]
    (apply str (mapv groups-as-strings (map #(mod (+ idx %) 5) (range 0 5))))
    )
  )

(defn -main
  "Euler problem 68"
  [& args]
  (last (filter #(= (count %) 16) 
                (into (sorted-set) (for [a (comb/permutations [1 2 3 4 5 6 7 8 9 10])
                                         :let [sum-for-a (sums a)]  
                                         :when (all-equal? sum-for-a)]
                                     (to-string a)
                                     ))))
  )

  ;; (let [rings (make-rings 3 6)]
  ;;   (loop [rings rings]
  ;;     (let [leaves (cset/difference ALL-NUMBERS (into (hash-set) cat (first rings)))]
        

  ;;       (if (not (zero? (count rings)))
  ;;         (recur (rest rings)))
  ;;       )
  ;;     )
  ;;   )
