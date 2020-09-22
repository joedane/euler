(ns e67.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn find-max-path-for-row
  ([values] (find-max-path-for-row values (- (count values) 2)))
  ([values row]
   ; this is not a very "clojurific" method of doing this ...
   (if (zero? row)
     (+ (aget values 0 0 )
        (max (aget values 1 0) (aget values 1 1)))
     (loop [col (dec (count (aget values row)))]
                                        ;    (pprint values)
       (if (< col 0)
         (find-max-path-for-row values (dec row))
         (do
           (aset values row col (+ (aget values row col) 
                                   (max (aget values (inc row) col)
                                        (aget values (inc row) (inc col)))))
           (recur (dec col))
           )         
         )
       )
     )
   )
  )

(defn -main
  "Euler prolem 67"
  [& args]
  (with-open [rdr (clojure.java.io/reader
                   (clojure.java.io/resource "p067_triangle.txt"))]
    (let [lines (line-seq rdr)
          values (to-array-2d 
                  (map (fn [line] 
                         (map (fn [s] (Integer/parseInt s)) (str/split line #" "))) 
                       lines))
          ]
      (find-max-path-for-row values)
      )

    )
  )
