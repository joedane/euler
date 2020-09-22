(ns e75.core
  (:require [euler-lib.core :as el])
  (:gen-class))


(defn is-triple? [a b c]
  (= (* c c) (+ (* a a) (* b b)))
  )

(defn triples [p]
  (for [a (range (/ p 3))
        b (range a (/ p 2))
        :let [c (- p (+ a b))]
        :when (is-triple? a b c)
        ]
    [ a b c]
    )
  )

(defn -main
  "Euler problem 75"
  [& args]
  (let [MAXP 1500000
        MAXM (first (el/exact-integer-sqrt (/ MAXP 2)))
        counts (int-array MAXP)
        ]
    (letfn 
        [(iter [m n]
;           (prn (format "%d %d" m n))
           (if (and (odd? (+ m n)) 
                    (= 1 (el/gcd m n))) 
             (let [a (- (* m m) (* n n))
                   b (* 2 m n)
                   c (+ (* m m) (* n n))
                   p (+ a b c)
                   ]
               (dorun
                (map #(aset-int counts % (inc (aget counts %))) (range p MAXP p)))
               ))
           (if (< (inc n) m)
             (recur m (inc n))
             (if (< (inc m) MAXM)
               (recur (inc m) 1))
             )
           )]
      (iter 2 1)
      )
    (count (filter #(= 1 %) (vec counts)))
    )
  )
