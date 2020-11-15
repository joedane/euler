(ns e76.core
  (:gen-class))

(def p_k
; see https://www.whitman.edu/mathematics/cgt_online/book/section03.03.html
  (memoize (fn [n k] 
             (cond (= n 0) 0
                   (= k 1) 1
                   (> k n) 0
                   :else
                   (+
                    (p_k (- n k) k)
                    (p_k (dec n) (dec k)))
                   )
             )
           )
  )

(defn p [n]
  (reduce + (map p_k (repeat n n) (range 1 (inc n)))))
  
(defn -main
  [& args]
  (dec (p 100)
       ))   

