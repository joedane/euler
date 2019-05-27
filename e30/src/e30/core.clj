(ns e30.core
  (:require [clojure.math.numeric-tower :as math]
            [clojure.string :as string])
  (:gen-class))

(defn number-to-digits [n]
  (map #(Integer/parseInt %) (string/split (format "%d" n) #""))
  )

(defn sum-of-powers-of-digits [n power]
  (->>
   (number-to-digits n)
   (map #(math/expt % power))
   (reduce +))
  )

(defn doit [power] 
  (->>
   (range 2 9999999)
   (filter #(= % (sum-of-powers-of-digits % power)))
   (reduce +)
   )
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))
