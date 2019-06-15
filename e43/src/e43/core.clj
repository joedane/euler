(ns e43.core
  (:require [clojure.math.combinatorics :as comb]
            [euler-lib.core :as el])
  (:gen-class))

(defn mk-num [l start]
  (+
   (* 100 (l start))
   (* 10 (l (inc start)))
   (l (+ 2 start))
   )
  )

(defn is-interesting-part? [l start divisor]
  (= 0 (rem (mk-num l start) divisor))
  )

(defn is-interesting? 
  "Argument is a list of 10 digits" 
  [l]
  (and 
   (is-interesting-part? l 1 2)
   (is-interesting-part? l 2 3)
   (is-interesting-part? l 3 5)
   (is-interesting-part? l 4 7)
   (is-interesting-part? l 5 11)
   (is-interesting-part? l 6 13)
   (is-interesting-part? l 7 17)
   )
  )

(defn list-to-number [l]
  (reduce + (map #(* %1 (el/expt 10 %2)) l (range (dec (count l)) -1 -1)))
  )

(defn -main
  "Euler problem 43"
  [& args]
  (reduce + 
          (map #(list-to-number %) (filter is-interesting? (comb/permutations (range 0 10)))))
  )
