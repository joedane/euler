(ns e23.core
  (:gen-class))

(defn natural-numbers
  ([] (natural-numbers 1))
  ([n] (cons n (lazy-seq (natural-numbers (+ n 1)))))
  )

(defn divisors [n]
  (for [i (range 1 (inc (/ n 2)))
        :when (= 0 (mod n i))] 
    i
    )
  )

(defn sum-of-divisors [n]
  (reduce + 0 (divisors n)))

(defn deficient? [n]
  (> n (sum-of-divisors n))
  )

(defn x-abundant? [n]
  (< n (sum-of-divisors n)))

(def abundant?
  (memoize (fn [n] (x-abundant? n))))

(defn abundant-numbers []
  (filter abundant? (natural-numbers))
  )

(defn is-sum-of-abundants [n]
  (loop [a (dec n)
         b 1]
    (if (and (> a 11) (abundant? a) (abundant? b))
      true
      (if (< a 12)
        false
        (recur (dec a) (inc b)))
      )
    )
  )

(defn numbers-that-are-not-the-sum-of-abundants []
  (filter #(not (is-sum-of-abundants %)) (take 23000 (natural-numbers)))
  )

(defn -main []
;  (println (reduce + 0 (numbers-that-are-not-the-sum-of-abundants)))
  (println (reduce + 0 (numbers-that-are-not-the-sum-of-abundants)) 
))
