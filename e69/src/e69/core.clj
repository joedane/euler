(ns e69.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn totient [n]
  (let [factors (el/factor n)]
    (* n 
       (reduce * (map #(- 1 (/ 1 %)) (keys factors)))
       ) 
    )
)

(defn -main
  "Euler problem 69"
  [& args]
  (loop [totients (for [n (range 2 1000001)
                        :let [t (totient n)]
                        ]
                    [n t (/ n t)]
                    )
         max-ratio 0.0
         max-n -1
         ]
    (if (empty? totients)
      max-n
      (let [this-t (first totients)]
        (if (> (nth this-t 2) max-ratio)
          (recur (rest totients) (nth this-t 2) (first this-t))
          (recur (rest totients) max-ratio max-n)
          )
        )    
      )
    )
  )
