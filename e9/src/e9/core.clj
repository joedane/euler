(ns e9.core
  (:require [clojure.math.numeric-tower :as math]))


(defn e9 []
  (for [a (range 1 1000)
        b (range 1 1000)
        c (range 1 1000)
        :when (and
               (= (* c c) (+ (* a a) (* b b)))
               (= 1000 (+ a b c)))]
    [a b c]))
