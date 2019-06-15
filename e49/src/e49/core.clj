(ns e49.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn split-digits [n]
  ((fn step [l n]
     (if (< n 10)
       (reverse (conj l n))
       (step (conj l (mod n 10)) (quot n 10))
       )
     ) [] n
   )
  )

(defn digits-to-number [l]
  ((fn step [l n]
     (if (zero? (count l))
       n
       (step (rest l) (+ n (* (first l) (el/expt 10 (dec (count l))))))
       )
     ) l 0)
  )

(defn prime-permutations [n]
  (let [digits (split-digits n)]
    (for [x (el/permutations digits)
          :when (= 4 (count x))
          :when (el/prime? (digits-to-number x))]
      x
      ))
  )

(defn make-permutations [start end]
  (->>
   (range start end)
   (filter el/prime?)
                                        ;(filter #(= 3 (count (prime-permutations %))))
   (map prime-permutations)
   (map #(map digits-to-number %))
   (map sort)
   ))

(defn -main
  "Euler problem 49"
  [& args]
  (let [l (make-permutations 1000 1020)]
    (for [l l]
      (map #(- %2 %1) l (rest l))
      )
    )
  )
