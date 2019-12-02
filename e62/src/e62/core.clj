(ns e62.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn count-digits [n]
  (inc (int (Math/floor (Math/log10 n))))
  )

(defn cubes 
  ([] (cubes 1))
  ([n] (cons (* n (* n n))
             (lazy-seq (cubes (inc n))))))

(defn encode [n]
  (let [s (format "%d" (el/coerce-unformattable-number n))
        f (frequencies s)]
    (format "%x%x%x%x%x%x%x%x%x%x"
            (get f \0 0)
            (get f \1 0)
            (get f \2 0)
            (get f \3 0)
            (get f \4 0)
            (get f \5 0)
            (get f \6 0)
            (get f \7 0)
            (get f \8 0)
            (get f \9 0)
            )
    )
  )

(defn find-matching-permutations [c cubes]
  (let [s (reduce
           (fn [m k]
             (update-in m [(encode k)] (fnil inc 0)))
           (hash-map)
           cubes
           )]
    (filter #(>= (second %) c) s)
    )
  )

(defn -main
  "Euler problem 62"
  [& args]
  
  )
