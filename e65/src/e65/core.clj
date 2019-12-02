(ns e65.core
  (:require [euler-lib.cf :as cf]
            [euler-lib.core :as el])
  (:gen-class))


; continued fraction representation of 'e'
(def cf-for-e 
  (letfn [(f [n]
            (cons 1 (cons (* 2 n) (cons 1 (lazy-seq (f (inc n))))))
            )]
    (cons 2 (f 1))
    ))

(defn -main
  "Projec Euler problem 65"
  [& args]
  (prn (reduce + (el/explode-digits (first (first (drop 99 (cf/cf->convergents cf-for-e))))))))
