(ns e21.core
  (:require [clojure.math.numeric-tower :as math]
            (:gen-class)))

(defn divisors [n]
  (for [i (range 1 (inc (/ n 2)))
        :when (= 0 (mod n i))] 
    i
    )
  )

(defn sum-of-divisors [n]
  (reduce + 0 (divisors n)))


(defn amicable []
  (for [a (range 1 10001)
        :let [b (sum-of-divisors a)]
        :when (not= a b)
        :when (> b 1)
        :when (= a
                 (sum-of-divisors b))]
    a)     
  )

(defn -main
  [& args]
  (println (reduce + 0 (amicable)))

  )
