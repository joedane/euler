(ns e31.core
  (:gen-class))

(def denominations '(200 100 50 20 10 5 2 1))

(defn make-combinations [amount denoms]
;  (printf "ways-to-make amt: %d\tdenoms: %s\n" amount (clojure.string/join " " denoms))
  (cond
    (< amount 0) '()
    (= amount 0) '([])
    (empty? denoms) '()
    (< amount (first denoms)) (make-combinations amount (rest denoms))
    (= (count denoms) 1) (if (= (rem amount (first denoms)) 0)
                           (list (into []  (take (quot amount (first denoms)) (repeat (first denoms)))))
                           '()
                           )
    :else 
    (concat
     (make-combinations amount (rest denoms))
     (map #(conj % (first denoms)) (make-combinations (- amount (first denoms)) denoms))
     )
    )
  )

(defn -main
  "Euler problem 31"
  [& args]
  (count (make-combinations 200 denominations)))
