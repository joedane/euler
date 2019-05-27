(ns e34.core
  (:require [clojure.string :as string])
  (:gen-class))

; what is the largest number we can form by adding N factorial digits?
; ans: NumDigits * 9!
;
; for example, with a 6 digit number, we can form 999999 -> 2,177,280
; with a 7 digit number, we can form 9999999 -> 2,540,160, a 7 digit number
; but with 8 digits, we can only go so high as 99999999 -> 2,903,040, also a
; 7 digit number.  So we can't ever be equal to even the smallest 8 digit 
; number (10,000,000).   So our upper bound is 7 digits.

(defn factorial [n]
  (cond
    (= n 0) 1
    (= n 1) 1
    :else 
    (* n (factorial (dec n)))
    )
  )

(def FACTORIAL-DIGITS
  (into [] (map factorial (range 0 10))))

(defn explode-digits [n]
  (let [s (format "%d" n)
        digits (string/split s #"")]
    (map #(Integer/parseInt %) digits)
    )
  )

(defn sum-of-factorial-digits [n]
  (reduce + (map FACTORIAL-DIGITS (explode-digits n)))
  )

(defn is-e34-number? [n]
  (= n (sum-of-factorial-digits n))
  )

(defn -main
  "Project Euler problem 34"
  [& args]
  (filter is-e34-number? (range 1 (* 7 (factorial 9))))
  )
