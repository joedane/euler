(ns e53.core
  (:gen-class))

(defn factorial
  ([n] (factorial n 1)) 
  ([n acc] (if (= n 0)
            acc
            (recur (dec n) (* acc n)))) 
  )

(def fm (memoize factorial))

(defn C [n r]
  (try 
    (/ (fm n) (* (fm r) (fm (- n r))))
    (catch Exception e (prn (format "Exception: %s [%d %d" (.getMessage e) n r)) 0)
    )
  )


(defn -main
  "Euler problem 53"
  [& args]
  (count (for [n (map bigint (range 1 101))
               r (map bigint (range 1 101))
               :when (>= n r)
               :let [Cnr (C n r)]
               :when (> Cnr 1000000)
               ]
           [n r Cnr]))
  )
