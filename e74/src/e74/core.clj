(ns e74.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(def FACTORIALS [1 1 2 6 24 120 720 5040 40320 362880])

(defn next-e74 [n]
  (reduce + (map FACTORIALS (el/explode-digits n)))
  )

(defn chain-length
  ([from] (chain-length from #{} 0)) 
  ([from seen len]
   (if (contains? seen from)
     len
     (chain-length (next-e74 from) (conj seen from) (inc len))
     )
   )
  )

(defn -main
  "Euler problem 74"
  [& args]
  (count (for [x (range 1 1000001)
               :let [len (chain-length x)]
               :when (= len 60)
               ]
           x)
         )
  )
