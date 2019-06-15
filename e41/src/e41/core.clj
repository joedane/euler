(ns e41.core
  (:require [euler-lib.core :as el]
            [clojure.string :as str]
            [clojure.math.combinatorics :as comb
             ])
  (:gen-class))


(defn -main
  "Project Euler problem 41"
  [& args]
  ; by experimentation, turns out 7 digits is the longest pandigital prime 
  (reduce max
          (for [x (map #(Long/parseLong (str/join %)) (comb/permutations [1 2 3 4 5 6 7]))
                :when (el/prime? x)
                ]
            x
            ))
  )
