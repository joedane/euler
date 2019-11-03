(ns e57.core
  (:gen-class))

(defn expand-sqrt2 [iterations]
  (let [r (/ 1 2)]
    (letfn [(iter [n]
              (if (= n 1)
                r
                (invert (+ 2 (iter (dec n)))))
              )
            ]
      (+ 1 (iter iterations))
      )
    )
  )

(defn count-digits [n]
                                        ;  (Math/ceil (Math/log10 n))
  (count (format "%s" n))
  )

(defn -main
  "Euler problem 57"
  [& args]
  (count (filter #(> (count-digits (numerator %)) (count-digits (denominator %)))
                 (for [n (range 1 1001)] (expand-sqrt2 n))))
  
  )
