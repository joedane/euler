
(ns e28.core
  (:gen-class))

(defn generate-differences []
  (letfn [
          (gen-n [count n]
            (if (> count 0)
              (lazy-seq (cons n (gen-n (dec count) n)))
              (gen-n 4 (+ n 2))
              )
            )
          ]
    (gen-n 4 2)
    )
  )

(defn generate-sums []
  (let [diffs (generate-differences)]
    (letfn [(get-next [n d]
              (lazy-seq (cons n (get-next (+ n (first d)) (rest d))))
              )]
      (get-next 1 diffs)
      )
    )
  )

(defn doit [n]
  (let [squares (int (/ n 2))
        nums (+ 1 (* 4 squares))]
    (reduce + (take nums (generate-sums)))
    )
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (doit)
  )
