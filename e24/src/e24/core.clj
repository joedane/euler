(ns e24.core
  (:gen-class))

(require '[clojure.string :as string])

(defn print-list [l]
  (println (string/join " "  l)))

(defn start-with [l i]
;  (assoc l 0 (l i) i (l 0))
  (vec (concat (vector (l i)) (subvec l 0 i) (subvec l (inc i))))
  )


(defn iterate-lexicographically [l y]
  (cond
    (= 0 (count l)) 'nil
    (= 1 (count l)) (y l)
    (= 2 (count l)) 
    (do 
      (y l)
      (y (reverse l)))
    :else 
    (loop [i 0]
      (if (< i (count l))
        (let [this-list (start-with l i)]
          (iterate-lexicographically (subvec this-list 1)
                                     (fn [tail]
                                       (y (concat (vector (first this-list)) tail))))  
          (recur (inc i))   
          )
        )
      )
    )
  )


(defn -main
  "24th Project Euler problem"
  [& args]
  (let [count (atom 1)
        count-fn (fn [l]
                   (if (= (deref count) 1000000) 
                     (print-list l))
                   (swap! count inc))]
    (iterate-lexicographically (vector 0 1 2 3 4 5 6 7 8 9) count-fn))
  )
