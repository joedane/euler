(ns e55.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn is-lychrel? [n] 
  (letfn 
      [(lychrel-step [n steps-remaining]
         (try
           (let [l (+ n (el/reverse-digits n))]
             (if (el/is-palindromic? l)
               false
               (if (> steps-remaining 0)
                 (lychrel-step l (dec steps-remaining))
                 true)
               ))
           (catch Exception e (do (prn n) (throw e)) 0)
           )
         ) 
       ]
    (lychrel-step n 50)
    )
  )

(defn -main
  "Euler problem 55"
  [& args]
  (count (filter is-lychrel? (range 1 10000)))
  )
