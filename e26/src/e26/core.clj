(ns e26.core
  (:gen-class))

; find the denominator <1000 that produces the maximum cycle length
; length of the cycle is the smallest i for which 10^i == 1 mod d

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

(defn remove-factors [n]
  ; remove powers of 2 and 5
  (letfn [(rm [n factor]
            (if (not= 0 (rem n factor))
              n
              (rm (/ n factor) factor))
            )]
    (rm (rm n 2) 5)
    )
  )

(defn modulo-power [base power mod]
  (loop [accum (rem base mod)
        p power]
    (if (= p 1)
      accum
      (recur (rem (* accum base) mod) (dec p))
      )
    )
  )

(defn cycle-length [p]
  ; p must be prime
  (loop [i 1]
    (if (= 1 (modulo-power 10 i p))
      i
      (recur (inc i))))
  )

(defn max-by-fn [f start]
  (loop [lst start
         n (first start)
         max (f (first start))
         ]
    (if (= (count lst) 1)
      [n max]
      (let [next-value (f (second lst))]
        (if (> next-value max)
          (recur (rest lst) (second lst) next-value)
          (recur (rest lst) n max))
        )
      )
    )
  )

(defn max-cycle-length-less-than [n]
  (->> 
   (range 2 n)
   (map #(remove-factors %))
   (filter #(prime? %))
   (max-by-fn cycle-length)
   )
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (max-cycle-length-less-than 1000))
)
