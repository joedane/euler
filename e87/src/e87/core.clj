(ns e87.core
  (:require [euler-lib.core :as el]
            [clojure.pprint])
  )



(def MAX-N 50000000)

(defn mk-seq [pow max]
  (letfn [(do-next [primes] (cons (el/expt (first primes) pow)
                                  (lazy-seq (do-next (rest primes)))))]
    (take-while #(< % max) (do-next (el/prime-numbers)))
    )
  )

(defn -main
  "Project Euler problem 87"
  [& args]
  ;; this doesn't quite do the trick, because some numbers can be represented in more than
  ;; one way.  Solved this by saving the sums to a file and doing a "< data.txt sort -nr | uniq | wc -l"
  (clojure.pprint/pprint
   (map #(apply + %)
        (for [q (mk-seq 4 MAX-N)
              c (mk-seq 3 MAX-N)
              s (mk-seq 2 MAX-N)
              :when (< (+ q c s) MAX-N)
              ]
          [q c s]
          )))
  )
