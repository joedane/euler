(ns e14.core
  (:gen-class))

(defn collatz [n]
  (if (= n 1)
    '(1)
    (cons n (lazy-seq (collatz (if (even? n) (/ n 2) (inc (* n 3))))))))

; this could obviouly be made quite a lot more efficient
(defn doit []
  (loop [longest-start -1
         longest-size -1
         n 1]
    (if (> n 1000000)
      [longest-start longest-size]
      (let [new-seq (collatz n)]
        (print (format "%d %d\t%d\n" n longest-start longest-size))
        (if (> (count new-seq) longest-size)
          (recur n (count new-seq) (inc n))
          (recur longest-start longest-size (inc n)))))))
  
  
(defn -main
  [& args]
  (doit)
  )
