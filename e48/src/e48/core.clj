(ns e48.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn n-to-the-n-mod-m [n m]
  ((fn step [multiples accumulator]
     (if (pos? multiples)
       (step (dec multiples) (mod (* accumulator n) m))
       accumulator)) 
   n 1)
  )

(defn add-modulo [a b m]
  (mod (+ a b) m)
  )

(defn -main
  "Euler problem 48"
  [& args]
  (reduce #(add-modulo %1 %2 (el/expt 10 10)) 
          (map #(n-to-the-n-mod-m % (el/expt 10 10)) (range 1 1001)))
  )
