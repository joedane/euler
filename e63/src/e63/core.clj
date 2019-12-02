(ns e63.core
  (:gen-class))

(defn pow [x p]
  (reduce * (repeat p (bigint x)))
  )


(defn count-digits [n]
  (inc (int (Math/floor (Math/log10 n))))
)

(defn digit-limit [x p]
  (not (< (count-digits (pow x p)) p))
  )

(defn digit-equals [x p]
  (= p (count-digits (pow x p)))
  )


(defn -main
  "Euler problem 63"
  [& args]
  (count
   (for [x (range 1 10)
         p (range)
         :while (digit-limit x p)
         :when (digit-equals x p)
         ]
     [x p]
     ))
  )
