(ns e79.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn index-of
  "return the index of char 'c' within array 'a', starting at index 'i', or nil if not found"
  [c a i]
  (loop [i i]
    (if (< i (count a))
      (if (= c (aget a i))
        i
        (recur (inc i))
        )
      )
    )
  )

(defn works? [test code]
  (if-let [a (index-of (nth test 0) code 0)]
    (if-let [b (index-of (nth test 1) code (inc a))]
      (if-let [c (index-of (nth test 2) code (inc b))]
        (> c b a)
        false
        )
      false
      )
    false
    )
  )

(defn works-for-all [tests code]
  (every? #(works? % code) tests )
  )

(defn to-code [n]
  (char-array (format "%d" n)))

(defn read-tests [filename]
  (with-open [rdr (clojure.java.io/reader (clojure.java.io/resource filename))]
    (set (map #(char-array %) (line-seq rdr)))
    )
  )


(defn -main
  "euler 79"
  [& args]
  (let [tests (read-tests "p079_keylog.txt")]
    (->>
     (el/naturals 1000)
     (map to-code)
     (filter (partial works-for-all tests))
     (first)
     )
    )
  )
