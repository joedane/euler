(ns e77.core
  (:require [euler-lib.core :as el])
  (:gen-class))

; see https://www.whitman.edu/mathematics/cgt_online/book/section03.03.html

(defn n_seq [n max]
  (take-while (partial >= max)
              (map (partial * n) (iterate inc 0)))) 

(defn p_from_seq [seq]
  (let [r (make-array Integer/TYPE (inc (apply max seq)))]
    (doseq [i (range (count seq))]
      (aset r (nth seq i) 1))
    r)
  )
  
(defn p_mult [p1 p2]
  (let [r (make-array Integer/TYPE (dec (+ (count p1) (count p2))))]
    (doseq [i (range (count p1))
            j (range (count p2))]
      (aset r (+ i j) (+ (aget r (+ i j))
                         (* (aget p1 i) (aget p2 j))))
      )
    r
    )
  )

(defn make-factors [max sequence]
  (map #(n_seq % max) (take-while (partial >= max) (sequence)))
  )

(defn make-polys [max sequence]
  (map p_from_seq (make-factors max sequence))
  )

(defn expand [n seq]
  (map dec ; Euler Problems 76 and 77 do not count "n" as a partition 
       (reduce p_mult (make-polys n seq))))

(defn -main
  ; 75 was found by trial and error
  [& args]
  (first (first 
          (filter #(and (<= 5000 (second %)) (> 5500 (second %))) 
                  (map-indexed vector 
                               (expand 75 el/prime-numbers)))))
  )
