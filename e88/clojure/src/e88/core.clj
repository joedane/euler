(ns e88.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn sums-to [l n]
  ( = n (reduce + l))
  )

(defn make-item-list [n k]
  (let [factors (el/factor n)]
    (letfn [(list-step [factors i])]
      (loop
          (if (< (inc i) (length factors))
            (lazy-seq (assoc (* (nth i factors))))
            ))
      )
    (cons factors (list-step factors 0))
    )
  )

(defn array-set! [a i v]
  (if (< i (alength a))
    (do (aset-int a i v)
        a)
    (let [new-array (int-array (* 2 (alength a) a))]
      (aset-int new-array i v)
      new-array
      )
    )
  )

(defn additive-partitions
  ([n] (additive-partitions n (int-array 50 0) 1))
  ([n avec m]
   (let [avec (array-set! avec m n)
         q (- m (if (= n 1) 1 0))]
     
     )
   (let [a0 0
         m 1
         q (- m (if (= n 1) 1 0))]
     (if (= 2 (nth v q))
       
     )
     
     )
   )
  )


(defn min-product-sum [k]
  
  (first
   (for [n (el/naturals 4)
         items (make-item-list n k)
         :when (sums-to items n)
         ]
     n
     )
   )
  )

(defn -main
  [& args]


  )
