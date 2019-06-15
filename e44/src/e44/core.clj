(ns e44.core
  (:gen-class))

(defn pentagonal-numbers
  ([] (pentagonal-numbers 1))
  ([n]
   (cons (/ (* n (- (* 3 n) 1)) 2)
         (lazy-seq (pentagonal-numbers (inc n)))))
  )

(def FIRST-TEN-THOUSAND-PENTAGONALS
  (into (hash-set) (take 10000 (pentagonal-numbers)))
  )

(defn is-pentagonal? [n]
  (contains? FIRST-TEN-THOUSAND-PENTAGONALS n)
  )

(defn -main
  "Euler problem 44"
  [& args]
  (for [p1 (take 10000 (pentagonal-numbers))
        p2 (take 10000 (pentagonal-numbers))
        :when (is-pentagonal? (+ p1 p2))
        :when (is-pentagonal? (Math/abs (- p1 p2)))
        ]
    [p1 p2]
    )
  )
