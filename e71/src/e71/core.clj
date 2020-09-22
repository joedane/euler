(ns e71.core
  (:gen-class))

(defn -main
  "Euler problem 71"
  [& args]
  
  (->>
   (for [
         d (map biginteger (filter #(not= 0 (mod % 7)) (range 2 1000000)))
         :let [
               n (biginteger (/ (* d 3) 7))
               ratio (clojure.lang.Ratio. n d)] 
         ]
     [ratio (- 3/7  ratio)] 
     )
   (apply min-key second)
   )  
  )
