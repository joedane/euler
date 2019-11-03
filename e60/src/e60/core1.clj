(ns e60.core
  (:require [clojure.math.combinatorics :as comb]
            [clojure.pprint :as pp]
            [euler-lib.core :as el])
  (:gen-class))

;; there are only two sets of 4 primes less than 1000 (an arbitrary limit) with the required character
;; (3 7 109 673) and (23 311 677 827)

(defn join-digits [n1 n2]
  (BigInteger. (str (format "%d" n1) (format "%d" n2)))
  )

(defn all-pairs-prime? [set]
  (every? (fn [pair] (and 
                      (el/prime? (join-digits (first pair) (second pair)))
                      (el/prime? (join-digits (second pair) (first pair)))))
          (comb/combinations set 2))
  )

(defn try-up-to [start n]
  (->> (for [new-prime (take-while #(< % n) (el/prime-numbers 1000))]
         (cons new-prime start)
         )
       (filter all-pairs-prime?)
       )
  )

(defn pairwise-prime? [a bs]
  (prn (format "%d %d" a bs))
  (and (el/prime? (join-digits a bs))
       (el/prime? (join-digits bs a)))
  )

(def not-pairwise-prime? (complement pairwise-prime?))

(def MAX-PRIME 1000)

(defn foo [n]
  (if (= n 2)
    (letfn [(prime-pairs [start-with remaining-primes]
             (let [next-prime (first (drop-while (partial not-pairwise-prime? start-with) remaining-primes))]
               (if (> next-prime MAX-PRIME)
                 (let [new-primes (el/prime-numbers start-with)]
                   (prime-pairs (first new-primes) (rest new-primes))
                   )
                 (cons [start-with next-prime]
                       (lazy-seq (prime-pairs start-with remaining-primes))))
               ))]
      (prime-pairs 2 (rest (el/prime-numbers)))
      )
    )
  )

(defn -main
  "Euler problem 60"
  [& args]
  (let [primes (take-while #(< % 10000) (el/prime-numbers))
        sets (comb/combinations primes 3)
        ]
    (pprint (filter all-pairs-prime? sets)
            (clojure.java.io/writer "three-primes.txt")) 
    )
  )
