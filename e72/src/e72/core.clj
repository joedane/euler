(ns e72.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn count-fractions [d]
  (->>
   (range 2 (inc d))
   (map el/totient)
   (reduce +))
  )

(defn -main
  "Euler problem 72"
  [& args]
  (count-fractions 8)
  )
