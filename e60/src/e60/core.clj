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
  (and (el/prime? (join-digits a bs))
       (el/prime? (join-digits bs a)))
  )

(defn prime-for-sublist? [l n]
  (every? #(pairwise-prime? % n) l)
  )

(def not-pairwise-prime? (complement pairwise-prime?))

(def MAX-PRIME 10000)

(defn primes-up-to 
  ([max]
   (primes-up-to 2 max)
   )
  ([start max] 
   (if (>= start max)
     []
     (take-while #(< % max) (el/prime-numbers start))))
  )

(defn add-to-sublist [n l]
  (conj l n)
  )

(defn foo [n]
  "Build up a list of 'pairwise prime' numbers, starting from pairs and adding one number at a time"
  (cond 
    (< n 2) (throw (IllegalArgumentException. "invalid input"))
    (= n 2)
    (letfn [(prime-pairs [start-with remaining-primes]
              (let [remaining-primes (drop-while (partial not-pairwise-prime? start-with) remaining-primes) 
                    next-prime (first remaining-primes)]
                (if (or (nil? next-prime) (> next-prime MAX-PRIME))
                  (let [new-primes (primes-up-to start-with MAX-PRIME)
                        new-start (first new-primes)]
                    (if (not (nil? new-start))
                      (recur (first new-primes) (rest new-primes))                      )
                    )
                  (cons [start-with next-prime]
                        (lazy-seq (prime-pairs start-with (rest remaining-primes)))))
                ))]
      (prime-pairs 2 (rest (primes-up-to MAX-PRIME)))
      )
    :else
    (letfn
        [(prime-list [sublists new-primes]
           (let [candidate-sublist (first sublists)
                 new-prime (first new-primes)]
             (if (not (nil? candidate-sublist))
               (if (nil? new-prime)
                 (recur (rest sublists) (primes-up-to MAX-PRIME))
                 (if (and (not (some #{new-prime} candidate-sublist)) ; contains? doesn't work here
                          (prime-for-sublist? candidate-sublist new-prime)
                          )
                   (cons (add-to-sublist new-prime candidate-sublist)
                         (lazy-seq (prime-list sublists (rest new-primes))))
                   (recur sublists (rest new-primes))
                   )
                 )
               )
             )
           )
         ]
      (prime-list (foo (dec n)) (primes-up-to MAX-PRIME))
      )
    )
  )

(defn -main
  "Euler problem 60"
  [& args]
  (reduce + (first (foo 5)))
  )
