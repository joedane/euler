(ns e5.core)

(def factors (range 2 21))

(defn check-multiple [n]
  (every? (fn [x] (= 0 (mod n x))) factors)
  )

(defn find-multiple []
  (loop
      [n 17]
    (if (check-multiple n)
      n
      (recur (+ n 17))))
  )
