(ns e50.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn longest-prime-sequence [start-with max]
  ((fn step [primes working-list longest-list sum]
     (let [next-sum (+ sum (first primes))
           next-list (cons (first primes) working-list)
           ]
       (if (> next-sum max)
         longest-list
         (recur (rest primes)
                next-list
                (if (el/prime? next-sum)
                  next-list
                  longest-list)
                next-sum
                ))
       )     
     ) (el/prime-numbers start-with) [start-with] [start-with] start-with)
  )

(defn longest-over-all-starts [max]
  ((fn step [starts longest]
     (let [seq (longest-prime-sequence (first starts) max)]
       (if (> (reduce + seq) max)
         longest
         (recur (rest starts)
                (if (> (count seq) (count longest))
                  seq
                  longest))
         )
       )
     ) (el/prime-numbers) []))

(defn -main
  "Euler problem 50"
  [& args]
  (reduce + (longest-over-all-starts 1000000))
  )

