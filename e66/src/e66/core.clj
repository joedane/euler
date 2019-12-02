(ns e66.core
  (:require [euler-lib.core :as el]
            [euler-lib.cf :as cf])
  (:gen-class))



(defn -main
  "Project Euler problem 66"
  [& args]
  (->> 
   (for [n (range 2 14) ; generate [D x y]  http://mathworld.wolfram.com/PellEquation.html
         :when (not (zero? (second (el/exact-integer-sqrt n))))
         :let [cf (cf/get-continued-fraction-for-sqrt n)
               r (- (count cf) 2)
               cf-digits (cf/continued-fraction-digits cf)
               convergents (cf/cf->convergents cf-digits)
               target-convergent (if (odd? r) 
                                   (nth convergents r)
                                   (nth convergents (inc (* 2 r))))
               ]
         ]
     [n (first target-convergent) (second target-convergent)]
     )
   (max-key second)
   )
  )
