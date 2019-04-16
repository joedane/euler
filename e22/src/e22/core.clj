(ns e22.core
  (:gen-class))



(defn read-next-name [r sep]
  (loop [readValue (.read r)
         buf []]
    (if (and (>= readValue 0) (not= (char readValue) sep))
      (recur (.read r)
             (if (= (char readValue) \")
               buf
               (conj buf (char readValue))
               ))
      (if (> (count buf) 0)
        (apply str buf)
        nil
        )
      )
    )
  )

(defn names-from-reader [r sep]
  (when-let [name (read-next-name r sep)]
    (cons name (lazy-seq (names-from-reader r sep))))
  )

(defn letter-value-sum [word]
  (reduce + 0 (map #(- (int %) 64) (seq word)))
  )

(defn e22 []
  (with-open [rdr (clojure.java.io/reader (clojure.java.io/resource "p022_names.txt"))]
    (let [names (sort (names-from-reader rdr \,))]
      (loop [sum 0
             pos 1
             names names]
        (if (seq names)
          (do
;            (printf "%s %d\n" (first names) (letter-value-sum (first names)))
            (recur (+ sum (* pos (letter-value-sum (first names))))
                   (inc pos)
                   (rest names)))
          sum
          )
        )
      )
    )
  )

(defn -main []
  (println (e22))
  )
