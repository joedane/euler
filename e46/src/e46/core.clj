(ns e46.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn is-perfect-square [n]
  (let [[s r] (el/exact-integer-sqrt n)]
    (= r 0))
  )

(defn goldbach2-counterexample? [n]
  "Return true if the argument (an odd composite number) does NOT satisfy the conjecture"
  ((fn step [maybe-prime] 
     (cond
       (= 1 maybe-prime) true
       (and (el/prime? maybe-prime)
            (is-perfect-square (/ (- n maybe-prime) 2))) false
       :else (step (- maybe-prime 2))
       )
     ) (- n 2))
  )

(defn -main
  "Euler problem 46"
  [& args]
  (first
   (for [n (el/nth-naturals 2)
         :when (not (el/prime? n))
         :when (goldbach2-counterexample? n)
         ]
     n
     ))
  )
