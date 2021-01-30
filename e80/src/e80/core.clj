(ns e80.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn is-digit [s]
  (Character/isDigit s)
  )

(defn to-digit [s]
  (Character/digit s 10)
  )

(defn debug [x]
  (prn x)
  x)

(defn sum-digits [str]
  (->>
   (seq str)
   (filter is-digit)
   (map to-digit)
   (take 100)
   (reduce +)
   )
  )

(defn -main
  "Euler problem 80"
  [& args]

  (with-open [rdr (clojure.java.io/reader (clojure.java.io/resource "sqrts.txt"))]
    (->>
     (line-seq rdr)
     (filter #(> (count %) 2))
     (map sum-digits)
     (debug)
     (reduce +)
     )
    )
  )
