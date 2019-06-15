(ns e36.core
  (:require [clojure.string :as string]
            [clojure.math.numeric-tower :as math]
            )
  (:gen-class))

(defn digit-at [n digit base]
  (quot (rem n (math/expt base digit)) (math/expt base (dec digit)))
  )

(defn is-palindrome? [n base]
  (let [num-digits (int (inc (/ (Math/log n)
                                (Math/log base))))
        half-num-digits (inc (int (Math/ceil (/ num-digits 2))))
        ]
    (every? #(= (digit-at n % base)
                (digit-at n (inc (- num-digits %)) base)
                )
            (range 1 half-num-digits))  
    )
  )


(defn -main
  "Euler problem 36"
  [& args]
  (reduce +
          (->> (range 1 1000000)
               (filter #(is-palindrome? % 10))
               (filter #(is-palindrome? % 2))))
  )
