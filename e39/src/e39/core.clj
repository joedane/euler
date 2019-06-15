(ns e39.core
  (:gen-class))

(defn is-pythagorean-triple? [a b c]
  (= (* c c) (+ (* a a) (* b b)))
  )

(defn -main
  "Project Euler problem 39"
  [& args]
  (filter #(> (count (second %)) 7)
   (group-by #(reduce + %)
             (for [c (range 1 1000)
                   b (range 1 c)
                   a (range 1 b)
                   :when (<= (+ a b c) 1000)
                   :when (is-pythagorean-triple? a b c)
                   ]
               [ a b c ])))
  )
