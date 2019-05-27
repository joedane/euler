(ns e29.core
  (:require [clojure.math.numeric-tower :as math])
  (:gen-class))

(defn unique-powers [lower upper]
  (into (hash-set) 
        (for [a (range lower upper) 
              b (range lower upper)]
          (math/expt a b)
          )
        )
  )

(defn doit [lower upper] 
  (count (unique-powers lower (inc upper)))
  )

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (format "%d" (doit 2 100))))
