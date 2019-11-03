(ns e56.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn sum-digits [n]
  (reduce + (el/explode-digits n))
  )

(defn -main
  "Euler problem 56"
  [& args]

  (letfn 
      [(max-of-list [a b]
         (if (> (nth a 2) (nth b 2))
           a
           b)
         )]
    (reduce max-of-list
            (for [a (range 1 10)
                  b (range 1 10)]
              [a b (sum-digits (el/expt a b))]
              )))
  )
