(ns e61.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn leading-digits [n]
  (let [s (format "%d" n)]
    (if (not (= (count s) 4))
      (throw (IllegalArgumentException. (format "bad number: %d" n))))
    (subs s 0 2)
    )
  )

(defn trailing-digits [n]
  (let [s (format "%d" n)]
    (if (not (= (count s) 4))
      (throw (IllegalArgumentException. (format "bad number: %d" n))))
    (subs s 2 4)
    )
  )

(defn take-4-digits [l]
  (take-while 
   #(< % 10000)
   (drop-while
    #(< % 1000) 
    l)
   )
  )

(defn make-series [generator]
  (let [l (take-4-digits generator)]
    (group-by leading-digits l)
    )
  )

(defn square-numbers
  ([] (square-numbers 1))
  ([n] (cons (* n n) (lazy-seq (square-numbers (inc n)))))
  )

(def all-lists {
                "triangular" (make-series (el/triangular-numbers)) 
                "square"     (make-series (square-numbers))
                "pentagonal" (make-series (el/pentagonal-numbers))
                "hexagonal"  (make-series (el/hexagonal-numbers))
                "heptagonal" (make-series (el/heptagonal-numbers))
                }
  )

(defn filter-on-candidate [lists start]
  ; return a list of two element vectors, one list item for each number in all lists
  ; that "match" the start value.  First vector element is the name of the list;
  ; second element is the number
  (let [match-digits (trailing-digits start)]
    (for [k (keys lists)
          v (get-in lists [k match-digits])]
      [k v]
      )
    )
  )

(defn try-path-from 
  [lists partial-path]
  (if (empty? lists)
     (if (= (leading-digits (first partial-path)) (trailing-digits (last partial-path)))
       partial-path    
       )
     (loop [maybe-next-list (filter-on-candidate lists (last partial-path))]
       (if (not (empty? maybe-next-list))
         (let [[list-name item] (first maybe-next-list)
               complete-path (try-path-from (dissoc lists list-name)
                                            (conj partial-path item))]
           (if (not (nil? complete-path))
             complete-path
             (recur (next maybe-next-list))
             )
           )
         )
       )
     )
  )


(defn -main
  "Euler projet 61"
  [& args]
  (let [root-list (take-4-digits (el/octagonal-numbers))]
    (reduce + (first (filter #(not (nil? %)) (map #(try-path-from all-lists [%]) root-list))))
    )
  )


