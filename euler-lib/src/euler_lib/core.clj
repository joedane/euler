(ns euler-lib.core
  (:require [clojure.math.numeric-tower :as math]
            [clojure.string :as string]
            [clojure.math.combinatorics :as comb]
            )
  )

(def expt math/expt)
(def exact-integer-sqrt math/exact-integer-sqrt)
(def permutations comb/permutations)

(defn prime? [n]
  "Simple inefficient primality test"
  (cond 
    (= n 0) false
    (= n 1) false
    (= n 2) true
    (< n 0) (prime? (* -1 n))
    :else 
    (let [stop (Math/sqrt n)]
      (loop [i 2]
        (if (= 0 (rem n i))
          false
          (if (> (inc i) stop)
            true
            (recur (inc i)))))
      )    
    )
  )

(defn prime-numbers
  "Prime numbers greater than n"
  ([] (prime-numbers 1))
  ([n]
   (let [next-prime (first (drop-while #(not (prime? %)) (iterate inc (inc n)))) ]
     (cons next-prime (lazy-seq (prime-numbers next-prime)))
     )
   ))

(defn factor [n]
  ((fn step [n factors try]
     (cond
       (> (* (first try) (first try)) n) (update factors n (fnil inc 0))
       (zero? (rem n (first try))) (step (quot n (first try)) 
                                         (update factors (first try) (fnil inc 0))
                                         try)
       :else (step n factors (rest try)) 
       )
     )
   n (hash-map) (prime-numbers)
   )
  )


(defn naturals
  ([] (naturals 1))
  ([n] 
   (cons n (lazy-seq (naturals (inc n)))))
  )

(defn nth-naturals
  ([inc] (nth-naturals 1 inc))
  ([start inc]
   ((fn step [n] (cons n (lazy-seq (step (+ n inc))))) start))
  )


(defn triangular-numbers
  ([] (triangular-numbers 1))
  ([n] 
   (cons (/ (* n (inc n)) 2) (lazy-seq (triangular-numbers (inc n))))))


(defn pentagonal-numbers
  ([] (pentagonal-numbers 1))
  ([n]
   (cons (/ (* n (- (* 3 n) 1)) 2)
         (lazy-seq (pentagonal-numbers (inc n)))))
  )


(defn is-pentagonal? [n]
  "Derrived from Wikipedia"
  (let [[s r] (math/exact-integer-sqrt (+ 1 (* n 24)))]
    (and (= 0 r)
         (= 5 (rem s 6)))
    )
  )

(defn hexagonal-numbers
  ([] (hexagonal-numbers 1))
  ([n]
   (cons (* n (- (* 2 n) 1))
         (lazy-seq (hexagonal-numbers (inc n)))))
  )

(defn is-hexagonal? [n]
  (let [[s r] (math/exact-integer-sqrt (+ 1 (* 8 n)))]
    (and (= 0 r)
         (= 3 (rem s 4)))
    )
  )

(def CHECK-PANDIGITAL (into (hash-set) (string/split "123456789" #"")))

(defn is-pandigital 
  "Confusingly, number and types vary across problems"
  ([a b] 
   (is-pandigital (format "%d%d%d" a b (* a b))))
  ([s] 
   (and (= 9 (count s))
        (= CHECK-PANDIGITAL (into (hash-set) (string/split s #"")))))
  )
