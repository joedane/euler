(ns e73.core
  (:gen-class))

(defn -main
  "Euler problem 73"
  [& args]
  (->>
   (for [d (range 2 12001)
         n (range (int (Math/ceil (/ d 3))) (inc (int (Math/floor (/ d 2)))))
         :let [r (clojure.lang.Ratio. (biginteger n) (biginteger d))]
         :when (and (> r 1/3) (< r 1/2))
         ]
     r
     )
   (filter #(= % (/ % 1))) ; clojure 1.10, Ratio r == (/ r 1) iff r is in lowest terms
   (count)
   )
  )
