(ns e4.core (:require [clojure.string :as s]))

(defrecord Factors [a b])

(defn is-palindromic [n]
  (let
      [n-fmt (format "%d" n)]
    (= n-fmt (s/reverse n-fmt))
      )
  )

(defn e4 []
  (apply max
         (for [a (range 1 1000)
               b (range 1 1000)
               :let [n (* a b)]
               :when (is-palindromic n)]
           n))
  )
