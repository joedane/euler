(ns e45.core
  (:require [euler-lib.core :as el])
  (:gen-class))

(defn -main
  "Euler problem 45"
  [& args]
  (first (filter #(and (el/is-pentagonal? %) (el/is-hexagonal? %)) (el/triangular-numbers 286)))
  )
