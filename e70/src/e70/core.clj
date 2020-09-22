(ns e70.core
  (:require [euler-lib.core :as el]
            [clojure.math.combinatorics :as comb]
            )
  (:gen-class))

(defn is-permutation? [a b]
  (= (frequencies (str a))
     (frequencies (str b)))
  )

(defn this-took-too-long []
  (loop [candidates (el/naturals 2)
         min-ratio 10000.0
         min-n -1
         ]
    (let [n (first candidates)]
      (if (> n 10000000)
        min-n
        (let [t (el/totient n)
              ratio (/ n t)]
          (if (and (< ratio min-ratio)
                   (is-permutation? n t)
                   )
            (recur (rest candidates) ratio n)
            (recur (rest candidates) min-ratio min-n)))))
    ))

(defn this-also-took-too-long []
(loop [
         candidates (for [pair (comb/combinations (take-while #(<= % 5000000) (el/prime-numbers)) 2)
                          :let [n (* (first pair) (second pair))
                                totient (* n (reduce * [(- 1 (/ 1 (first pair))) (- 1 (/ 1 (second pair)))]))]
                          :when (< n 10000000)
                          :when (is-permutation? n totient)
                          ] 
                      [n totient]
                      )
         min-ratio 1000.0
         min-n -1
         ]
    (if (empty? candidates)
      min-n
      (let [[n t] (first candidates)]
        (if (< (/ n t) min-ratio)
          (recur (rest candidates) (/ n t) n)
          (recur (rest candidates) min-ratio min-n)
          )
        ))
    )
  )

(defn -main
  "Euler problem 70"
  [& args]
  (for [factor-one (reverse (take-while #(< % (Math/sqrt 10000000)) (el/prime-numbers)))
        factor-two (reverse (take-while #(< % factor-one) (el/prime-numbers)))
        :let [n (* factor-one factor-two)
              totient (* n (reduce * [(- 1 (/ 1 factor-one)) (- 1 (/ 1 factor-two))])) 
              ]
        :when (is-permutation? totient n)
        ]
    [n totient (double (/ n totient))]
    )
  )

; 8319823
