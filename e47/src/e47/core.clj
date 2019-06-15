(ns e47.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn drop-until-sequential [n]
  (fn [rf]
    (let [sequence-base (volatile! 0)
          count (volatile! 0)]
      (fn
        ([] (rf))
        ([result] (rf result))
        ([result input]
         (if (= (input 0) (+ @sequence-base @count))
           (if (= (inc @count) n)
             (reduced @sequence-base)
             (do (vswap! count inc) result))
           (do
             (vreset! sequence-base (input 0))
             (vreset! count 1)
             result
             ))
         )
        )
      )
    )
  )

(defn -main
  "Euler problem 47"
  [& args]
  (transduce (drop-until-sequential 4) 
             conj 
             (->>
              (for [n (el/naturals)]
                [n (count (el/factor n))]
                )
              (filter #(= (% 1) 4))
              ))
  )
