(ns e18.core
  (:require [clojure.math.numeric-tower :as math])
  (:gen-class))


(def m (to-array-2d
        [
         [75]
         [95 64]
         [17 47 82]
         [18 35 87 10]
         [20 4 82 47 65]
         [19 1 23 75 3 34]
         [88 2 77 73 7 63 67]
         [99 65 4 28 6 16 70 92]
         [41 41 26 56 83 40 80 70 33]
         [41 48 72 33 47 32 37 16 94 29]
         [53 71 44 65 25 43 91 52 97 51 14]
         [70 11 33 28 77 73 17 78 39 68 17 57]
         [91 71 52 38 17 14 91 43 58 50 27 29 48]
         [63 66 4 68 89 53 67 30 73 16 69 87 40 31]
         [4 62 98 27 23 9 70 98 73 93 38 53 60 4 23]
         ]))
  
;; (def m (to-array-2d
;;         [
;;          [3]
;;          [7 4]
;;          [2 4 6]
;;          [8 5 9 3]
;;          ]
;;         ))

(defn choice-at-step [n step]
  (loop [choice 0
         n n
         step step]
    (if (= step 0)
      choice
      (recur (+ choice (bit-and n 2r1)) (bit-shift-right n 1) (dec step))))
  )
      
(defn path-for-n [n]
;  (list n (+ n 1) (+ n 2))
  (for [level (range (alength m))]
    (aget m level (choice-at-step n level))
    )
  )


; return a sequence of integers
(defn sum-seq
  []
  (map #(reduce + 0 %)
       (for [n (range (math/expt 2 (alength m)))]
         (path-for-n n)
         )
       )
  )

(defn -main
  [& args]
  (println (format "%d" (apply max (sum-seq)))) 
  )
