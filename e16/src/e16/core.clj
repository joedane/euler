(ns e16.core
  (:require [clojure.math.numeric-tower :as math])
  (:gen-class))


(defn sum-digits [n]
  (let [n-str (format "%d" (biginteger n))]
    (loop [acc 0 pos 0]
      (if (= pos (count n-str))
        acc
        (recur (+ acc (Character/digit (get n-str pos) 10)) (inc pos))))))

(defn -main
  [& args]
  (print (sum-digits (math/expt 2 1000))))
  
