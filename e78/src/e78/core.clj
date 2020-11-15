(ns e78.core
  (:require [euler-lib.core :as el])
  (:gen-class))


(defn euler-signs 
  ([] (euler-signs -1 1))
  ([last next]
   (cons next
         (lazy-seq (euler-signs next
                                (if (= last next) 
                                  (* -1 next)
                                  next
                                  )
                                )
                   )
         )
   )
 )

(defn euler-k
  ([] (euler-k 1))
  ([next]
   (cons next
         (lazy-seq (euler-k (if (< next 0) (inc (-' next)) (-' next))))))
  )

(defn euler-pentagonal-numbers []
  (map #(/ (* % (- (* 3 %) 1)) 2) (euler-k))
  )

(defn euler-sums [n]
  (take-while (partial >= n) (euler-pentagonal-numbers))
  )

                                        ; this was way too slow using bignums.
                                        ; took a hint from the net re storing only the remainder mod 1000000
(defn compute-partition-number-tail [p] 
  (cond (= p 0) 1
        (= p 1) 1
        :else
        (mod (reduce +
                     (map * 
                          (euler-signs)
                          (map partition-number-tail
                               (map #(- p %) (euler-sums p))
                               )))
             1000000
             )
        )
  )

(def partition-number-tail (memoize compute-partition-number-tail))



(defn -main
  "Euler problem 78"
  [& args]
  (->>
   (for [n (el/naturals)]
     [n (partition-number-tail n)])
   (filter #(zero? (mod (second %) 1000000)))
   (first)
   (first)
   )
  )
