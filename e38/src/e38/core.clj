(ns e38.core
  (:gen-class)
  (:require [euler-lib.core :as el])
  )

; assume that the largest number must be larger than that given by the product
; of 9 and (1, 2, 3, 4, 5) given in the problem statement.  The integer can't
; be a two digit number starting with 9, since we can't make a 9 digit number.
; so it must be a three digit number starting with 9 (so that the first digit
; will be a nine and the number can be larger than the number given in the 
; problem statement).

(defn concatenate-digits [l]
  (reduce str
          (map #(format "%d" %) l) 
          )
  )

(defn max-digits [a b]
  (let [c (compare a b)]
    (if (> c 0)
      a
      b)
    )
  )

(defn -main
  "Project Euler problem 38"
  [& args]
  (reduce max-digits
          (filter el/is-pandigital
                  (let [multipliers '(1 2)]
                    (for [x (range 9000 10000)]
                      (concatenate-digits (map #(* x %) multipliers))
                      )
                    )))
  )
