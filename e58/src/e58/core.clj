(ns e58.core
  (:require [euler-lib.core :as el])
  (:gen-class))

; 1  2  3  4  5   6   7   8  9  10  11  12  13
; 1  3  5  7  9  13  17  21 25  31  37  43  49
;  2   2  2  2  4   4  4   4  6   6   6   6  

(defn diagonals 
  ([] (diagonals 1 2 3))
  ([next step cnt]
   (cons next (lazy-seq 
               (diagonals (+ next step) 
                          (if (zero? cnt) (+ step 2) step)
                          (if (zero? cnt) 3 (dec cnt))))))
  )

(defn diagonals-with-extra
  ([] (diagonals-with-extra 1 0 1 2 3))
  ([n primes-seen next step cnt]
   (cons [n primes-seen next] 
         (lazy-seq
          (diagonals-with-extra (inc n)
                                (if (el/prime? next) (inc primes-seen) primes-seen)
                                (+ next step)
                                (if (zero? cnt) (+ step 2) step)
                                (if (zero? cnt) 3 (dec cnt))
                                )))
   )
  )

(defn side-length [n]
  "Given the sequence number of a given diagonal, return the legnth of the corresponding 'square'"
  (inc (* 2 (int (Math/ceil (/ (dec n) 4)))))
  )


(defn -main
  "Euler problem 58"
  [& args]
  (let 
      [n (first (drop-while (fn [v] (>= (/ (v 1) (v 0)) 0.1)) (drop 2 (diagonals-with-extra))))]
    (side-length (n 0))
    )
  )


(defn -main2
  "Euler problem 58 (not working -- off by a few)"
  [& args]
  (loop [diags (rest (diagonals))  ; skip the initial '1'
         n 2
         primes-seen 0
         next (first diags)
         ]
    (if (el/prime? next)
      (recur (rest diags)
             (inc n)
             (inc primes-seen)
             (first diags))
      (if (< (/ primes-seen n) 0.1)
        (side-length n)
        (recur (rest diags)
               (inc n)
               primes-seen
               (first diags))
        )
      ) 
    )
  )
