(ns e27.core
  (:gen-class))

(defn prime? [n]
  (if (> n 1)
    (let [stop (Math/sqrt n)]
      (loop [i 2]
        (if (= 0 (rem n i))
          false
          (if (> (inc i) stop)
            true
            (recur (inc i)))))
      )
    false
    )
  )

(defn eval-quadratic [cl const x]
  (+ (* x x)
     (* cl x)
     const)
  )

(defn make-sequence []
  (for [a (range -999 1000)
        b (range -1000 1001)
        ]
    [a b (for [x (range 0 20000) 
               :let [value (eval-quadratic a b x)]
               :while (prime? value)]
           value
           )]
    )
  )

; this will pick out the combinations with "count" > 30.  Will look for the largest amoung the bunch
(defn analyze-sequence [s]
  (for [item s :when (> (count (nth item 2)) 30)] 
    [(first item) (second item) (count (nth item 2))]
    )
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  
  )
