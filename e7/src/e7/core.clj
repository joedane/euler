(ns e7.core)

(defn gen-primes "Generates an infinite, lazy sequence of prime numbers"
  []
  (let [reinsert (fn [table x prime]
                   (update-in table [(+ prime x)] conj prime))]
    (defn primes-step [table d]
                 (if-let [factors (get table d)]
                   (recur (reduce #(reinsert %1 d %2) (dissoc table d) factors)
                          (inc d))
                   (lazy-seq (cons d (primes-step (assoc table (* d d) (list d))
                                                 (inc d))))))
    (primes-step {} 2)))

(defn e7
  (nth (gen-primes) 10000))
