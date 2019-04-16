(def max-fib 4000000)

(defn add-even-fibs [last sum]
  (if (>= (get last 0) max-fib)
    sum
    (let [next-fib (+ (get last 0) (get last 1))]
      (add-even-fibs [(get last 1) next-fib]
                     (if (even? next-fib) (+ sum next-fib) sum)))))


(add-even-fibs [1 2] 2)
