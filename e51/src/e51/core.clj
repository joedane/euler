(ns e51.core
  (:require [euler-lib.core :as el]
            [clojure.string :as string])
  (:gen-class))

; brute force, terribly inefficient, and difficult to read solution

(defn digits-to-list [n min-digits]
  (map #(Integer/parseInt %) 
       (string/split (format 
                      (str "%0" (format "%d" min-digits) "d") 
                      n) 
                     #""))
  )

(defn list-to-digits [l]
  ((fn step [l n]
     (if (zero? (count l))
       n
       (recur (rest l) (+ n (* (first l) 
                               (el/expt 10 (dec (count l))))))))
   l 0)
  )

(defn replacement-templates [digits-fixed digits-to-sub]
  "returns a list of \"replacment templates\", i.e. for input 2 1, the following:
((0 0 replace)
 (0 replace 0)
 (replace 0 0)
 (0 1 replace)
 (0 replace 1)
 (1 0 replace)
  ...
"
  (->>
   (range 0 (el/expt 10 digits-fixed))
   (map #(digits-to-list % digits-fixed))
   (map #(concat % (take digits-to-sub (repeat 'replace))))
   (map el/permutations)
   (apply concat)
   )
  )

(defn instantiate-template [l]
  (letfn [(maybe-replace [l n]
            (map #(if (= % 'replace) n %) l)
            )]
    (for [a (range 0 10)
          :let [number (list-to-digits (maybe-replace l a))] 
          :when (el/prime? number)
          ]
      number
      ))
  )

(defn -main
  "Euler problem 51"
  [& args]
  (filter #(<= 8 (count %)) (map instantiate-template (replacement-templates 3 3)))  
  )
