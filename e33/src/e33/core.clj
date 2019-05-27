(ns e33.core
  (:gen-class))

(defn make-all-posible-fractions []
  (for [d (range 10 100)
        n (range 10 d)]
    [n d]
    ))

(defn reducable? [n d]
;  (println "reduce %d %d" n d)
  (let [nu (rem n 10)
        nt (quot n 10)
        du (rem d 10)
        dt (quot d 10)]
    (if (not= 0 (+ nu du))
      (try
        (cond
          (= nu du) (/ nt dt)
          (= nu dt) (/ nt du)
          (= nt du) (/ nu dt)
          (= nt dt) (/ nu du)
          :else false
          )
        (catch ArithmeticException e false)
        )
      false
      )
    )
  )

(defn special? [n d]
  (if-let [r (reducable? n d)]
    (= r (/ n d))
    false
    )
  )

(defn -main
  [& args]
  (filter #(special? (first %) (second %)) (make-all-posible-fractions))
  )
