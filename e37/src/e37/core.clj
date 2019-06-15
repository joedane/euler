(ns e37.core
  (:gen-class)
  (:require [euler-lib.core :as el])
  )

(defn all-primes-from-right? [n]
  (if (el/prime? n)
    (if (< n 10)
      true
      (all-primes-from-right? (int (/ n 10))))
    false
    )
  )

(defn all-primes-from-left? [n]
  (if (el/prime? n)
    (if (< n 10)
      true
      (all-primes-from-left? (rem n (el/expt 10 (int (Math/log10 n)))))
      )
    false
    )
  )

(defn -main
  "Project Euler problem 37"
  [& args]
  (reduce +
          (take 11
                (filter #(and (all-primes-from-right? %)
                              (all-primes-from-left? %))
                        (el/naturals 9)
                        )
                ))
  )
