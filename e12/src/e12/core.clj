(ns e12.core
  (:require [clojure.math.numeric-tower :as math])
  (:gen-class))

(defn triangular-numbers
  ([] (triangular-numbers 1 0))
  ([n last] (cons (+ n last) (lazy-seq (triangular-numbers (inc n) (+ n last))))))

(defn natural-numbers
  ([] (natural-numbers 1))
  ([n] (cons n (lazy-seq (natural-numbers (inc n))))))

(defn divisors [n]
  (for [x (range 1 (inc n))
        :when (= 0 (mod n x))]
    x))

(defn factor
  ([n] (factor n (hash-map)))
  ([n divisors]
     (loop [try 2 n n divisors divisors]
       (if (> try n)
         divisors
         (if (= 0 (mod n try))
           (recur try
                  (/ n try)
                  (merge-with + divisors {try 1}))
           (recur (inc try) n divisors))))))

(defn num-divisors [factors]
  (reduce * (map inc (vals factors))))

(def divide-by-two {2 -1})

(defn nth-triangle-number-factors [n]
  (merge-with + (factor n) (factor (inc n)) divide-by-two))

(defn doit [] 
; (some #(if (> (count (divisors %)) 500) %) (triangular-numbers))
  (some #(if (> (num-divisors (nth-triangle-number-factors %)) 500) %) (natural-numbers)))

  

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (doit))

