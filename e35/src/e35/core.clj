(ns e35.core
  (:require [clojure.string :as string])
  (:gen-class))

(defn prime? [n]
  (cond
    (= n 2) true
    (> n 1) (let [stop (Math/sqrt n)]
              (loop [i 2]
                (if (= 0 (rem n i))
                  false
                  (if (> (inc i) stop)
                    true
                    (recur (inc i)))))
              )
    :else false
    )
  ) 

(defn explode-digits [n]
  (let [s (format "%d" n)
        digits (string/split s #"")]
    (map #(Integer/parseInt %) digits)
    )
  )

(defn rotate [digits]
  (concat (rest digits) (list (first digits)))
  )

(defn rotate-seq [digits]
  (let [rotated (rotate digits)]
    (cons rotated (lazy-seq (rotate-seq rotated)))
    )
  )

(defn prime-rotations? [n]
  ; it is assmed that n itself is prime
  (let [digits (explode-digits n)]
    (every? prime? 
            (map
             #(Integer/parseInt (string/join %))
             (take (dec (count digits)) (rotate-seq digits))))
    )
  )

(defn -main
  "Project Euler problem 35"
  [& args]
  (->> (range 1 1000000)
       (filter prime?)
       (filter prime-rotations?)
       (count))
  )
