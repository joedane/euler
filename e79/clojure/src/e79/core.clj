(ns e79.core
  (:require [euler-lib.core :as el]
            [clojure.string :as str])
  (:gen-class))

(defn index-of
  "return the index of integer 'n' within vector 'v', starting at index 'i', or nil if not found"
  [n v i]
  (loop [i i]
    (if (< i (count v))
      (if (= n (nth v i))
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

(defn string-to-int-vector [s]
  (vec (map #(Integer/parseInt %) (str/split s #"")))
  )

(defn read-tests [filename]
  (with-open [rdr (clojure.java.io/reader (clojure.java.io/resource filename))]
    (set (map string-to-int-vector (take 4 (line-seq rdr)) ))
    )
  )

(defn make-code [parts new]
  (concat (first parts) [new] (second parts))
  )

(defn position-for [code c i]
  (loop [i i]
    (if (or (= i (count code)) (>= (nth code i) c))
      i
      (recur (inc i)))
    )
  )


(defn insert-starting-at [code c i]
  (let [pos (position-for code c i)]
    (make-code (split-at pos code) c)
    )
  )

(defn increment-code [code]
  
  )

(defn integrate-test [code test]
  (loop [code code
         tests-remaining (rest test)
         c (first test)
         start-at 0]
    
    (let [at (index-of c code start-at)]
      (if (nil? at)
        (let [newcode (insert-starting-at code c start-at)]
          (if (empty? tests-remaining)
            newcode
            (recur newcode (rest tests-remaining) (first tests-remaining) start-at))
          )
        (if (empty? tests-remaining)
          code
          (recur code (rest tests-remaining) (first tests-remaining) at)
          )
        )
      )
    )
  )

(defn -main
  "euler 79"
  [& args]
  (let [tests (read-tests "p079_keylog.txt")]
;    (reduce integrate-test tests) 
    
    )
  )
