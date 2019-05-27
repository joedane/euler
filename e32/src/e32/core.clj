(ns e32.core
  (:require [clojure.string :as string])
  (:gen-class))

(defn has-right-number-of-digits [a b]
  (let [digitsA (int (Math/ceil (Math/log10 a)))
        digitsB (int (Math/ceil (Math/log10 b)))
        d (* 2 (+ digitsA digitsB))]
    (= d 10)
    )
  )

(def CHECK (into (hash-set) (string/split "123456789" #"")))


(defn is-pandigital [a b]
  (let [s (format "%d%d%d" a b (* a b))]
    (and (= 9 (count s))
         (= CHECK (into (hash-set) (string/split s #""))))
    )
  )

(defn all-pandigitals []
  (for [a (range 1 9999)
        b (range 1 9999)
        :when (and 
               (has-right-number-of-digits a b)
               (is-pandigital a b))]
   ; [a b (* a b)]
    (* a b)
    )
  )

(defn -main
  [& args]
  (reduce + (into (hash-set) (all-pandigitals)))
  )
