(ns e40.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(def CHAR-TO-INT {\0 0
                  \1 1
                  \2 2
                  \3 3
                  \4 4
                  \5 5
                  \6 6
                  \7 7
                  \8 8
                  \9 9})

(defn digit-stream []
  ((fn step [numbers current-digits]
     (if (> (count current-digits) 1)
       (lazy-seq (cons (get CHAR-TO-INT (first current-digits)) (step numbers (rest current-digits))))
       (let [next-number (second numbers)]
         (lazy-seq (cons (get CHAR-TO-INT (first current-digits)) (step (rest numbers) (format "%d" next-number)))))
       )  
     ) (el/naturals) "1"
   )
  )

(defn -main
  "Project Euler problem 40"
  [& args]
  (let [digits (digit-stream)] 
    (reduce *
           [(nth digits 0)
            (nth digits 9)
            (nth digits 99)
            (nth digits 999)
            (nth digits 9999)
            (nth digits 99999)
            (nth digits 999999)]))
  )
