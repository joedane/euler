(ns e25.core
  (:gen-class))

(def MEMO (atom {1 (biginteger 1) 2 (biginteger 1)}))

(defn fib [n]
  (letfn [(fib-rec [n] 
            (if (contains? @MEMO n)
              (get @MEMO n)
              (let [fib-n (+ (fib-rec (dec n)) (fib-rec (dec (dec n))))]
                  (swap! MEMO
                         (fn [current] 
                           (assoc current n fib-n)
                           )) 
                  (println (format "fib(%d) has %f digits" n (Math/ceil (Math/log10 fib-n))))
                  fib-n)))]
      (fib-rec n)
      )
  )

(def PHI (/ (+ 1 (Math/sqrt 5)) 2))

(defn digits-in-nth-fibonacci [n]
  (int
   (Math/ceil
    (-
     (* n (Math/log10 PHI))
     (/ (Math/log10 5) 2))))
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (loop [n 1]
    (if (= 1000 (digits-in-nth-fibonacci n))
      (println n)
      (recur (inc n)))
    )
  )
