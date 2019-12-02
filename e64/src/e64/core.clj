(ns e64.core
  (:require [euler-lib.cf :as cf])
  (:gen-class))



(defn get-period [n]
  (dec (count (cf/get-continued-fraction-for-sqrt n)))
  )


(defn -main
  "Euler problem 64"
  [& args]
  (prn (count 
        (filter odd? (map get-period (range 2 10001)))))
  )
