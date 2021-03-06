(ns e3.core
  (:require [clojure.math.numeric-tower :as math]))

(def N (biginteger 600851475143))

(defn is-exact-sqrt [n]
  (let [[s r] (math/exact-integer-sqrt n)]
    (= r 0)))

(defn exact-sqrt-floor [n]
  (get (math/exact-integer-sqrt n) 0))

(defn foo [x]
  (is-exact-sqrt (- (* x x) N)))

(get (math/exact-integer-sqrt N) 1)


(defn factorize [n]
  (loop [factors #{}
         i 2]
      (if (= i (exact-sqrt-floor n))
        factors
        (let [d (biginteger(math/gcd i n))]
          (if (= d i)
            (recur (conj factors d) (+ i 1))
            (recur factors (+ i 1)))))))

