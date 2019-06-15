(ns e42.core
  (:require [clojure.java.io :as io]
            [clojure.string :as str]
            )
  (:gen-class))

(def CHAR-VALUES (zipmap "ABCDEFGHIJKLMNOPQRSTUVWXYZ" (range 1 27)))

(defn triangular-numbers
  ([] (triangular-numbers 1))
  ([n] 
   (cons (/ (* n (inc n)) 2) (lazy-seq (triangular-numbers (inc n))))))

(def COMPARE-SET (into (hash-set) (take-while #(<= % 364) (triangular-numbers))))

(defn get-words [file-name]
  (map #(subs % 1 (dec (count %))) (str/split (slurp (io/file file-name)) #","))
  )

(defn word-to-number [word]
  (reduce + (map CHAR-VALUES word))
  )

(defn -main
  "Project Euler problem 42"
  [& args]
  ; note that (by inspection) the longest word is 14 characters long, so the largest "word number" is 14*26 = 364
  (count
   (->>
    (get-words "resources/p042_words.txt")
    (map #(word-to-number %))
    (filter #(contains? COMPARE-SET %))
    ))
  )
