(ns e20.core
  (:require [clojure.math.numeric-tower :as math])
  (:gen-class))


(defn sum-digits [n]
  (let [n-str (format "%d" (biginteger n))]
    (loop [acc 0 pos 0]
      (if (= pos (count n-str))
        acc
        (recur (+ acc (Character/digit (get n-str pos) 10)) (inc pos))))))

(defn factorial [n]
  (if (= n 0)
    1
    (*' n (factorial (dec n)))))

(defn -main
  [& args]
  (print (sum-digits (factorial 100))))

