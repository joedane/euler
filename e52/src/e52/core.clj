(ns e52.core
  (:require [euler-lib.core :as el
             ] )
  (:gen-class))

(defn digits-in [n]  ; see doc page for update-in
  (reduce 
   (fn [m k]
     (update-in m [k] (fnil inc 0)))
   {}
   (format "%d" n))
  )

(defn same-digits [a b]
  (= (digits-in a) (digits-in b))
  )

(defn make-multiples [a]
  [a (* 2 a) (* 3 a) (* 4 a) (* 5 a) (* 6 a)]
  )

(defn -main
  "Euler problem 52"
  [& args]
  (first
   (->> (el/naturals 100000)
        (filter (fn [a]
                  (every? (fn [b]
                            (same-digits a b)) (make-multiples a))))
        ))
  
  )
