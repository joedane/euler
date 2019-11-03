(ns e59.core
  (:require [clojure.string :as str])
  (:require [clojure.math.combinatorics :as comb])
  (:gen-class))

;; Frequency analysis ...

;; (["80" 107]
;;  ["69" 86]
;;  ["88" 77]
;;  ["0" 75]
;;  ["17" 73]
;;  ["29" 70]
;;  ["21" 65]
;;  ["12" 65]
;;  ["4" 61]
;;  ["22" 56]
;;  ["10" 52]
;;  ["23" 46]
;;  ...)

;; ASCII 'a' == 97
;;       'z' == 122


(def most-frequent-codes
  '("80" "69" "88" "0" "17" "29" "21" "12" "4")
  )

(def try-these-chars 
  '(\space \e \t)
  )

;; (def try-these-keys 
;;   (for [codes (comb/combinations most-frequent-codes 3) 
;;         chars (comb/permutations try-these-chars)]
;;     (filter
;;      (fn [l] (every? (fn [n] (and (>= n 97) (<= n 122))) l))
;;      (map 
;;       (fn [code char] (bit-xor (Integer/parseInt code) (byte char)))
;;       codes 
;;       chars
;;       ))
;;     )
;;   )

(def all-keys 
  (let [mappings (for [codes (comb/combinations most-frequent-codes 3) 
                       chars (comb/permutations try-these-chars)]
                   [codes chars]
                   )]
    (map (fn [mapping]
           (apply map (fn [code char] (bit-xor (Integer/parseInt code) (byte char))) mapping)
           ) 
         mappings)
    ;; (apply map 
    ;;        (fn [codes chars] 
    ;;          (map #(bit-xor (Integer/parseInt %1) (byte %2)))
    ;;          )
    ;;        mappings)
    ;;    (filter
;;     (fn [l] (every? (fn [n] (and (>= n 97) (<= n 122))) l))
;;     )
;;    (map #(char %))
;; ;   (apply str)
;;    )
  ))

(def try-these-keys
  (->> all-keys
       (filter 
        (fn [l] 
          (every? (fn [n] (and (>= n 97) (<= n 122))) 
                  l)))
       (map (fn [l] (map char l)))
       (map #(apply str %))
       )
  )

(defn key-from [s]
  (if (= \- (first s))
    [0 0 0]
    (map #(byte %) (char-array s))
    )
  )

(defn map-cipher 
  [key cipher]
  (map (fn [cbyte kbyte] (char (bit-xor (byte cbyte) kbyte)))
       cipher
       (cycle key))
  )

(defn try-key [cipher key]
  (apply str
         (map-cipher (key-from key) cipher))
  )


(defn try-all-keys [cipher]
  (doseq [key try-these-keys]
    (prn (format "k: %s  %s" key (subs (try-key cipher key) 0 15)))
    )
  )

(defn -main
  "Project euler problem 59"
  [& args]
  (let [ascii (str/split 
               (slurp (clojure.java.io/reader (clojure.java.io/resource "p059_cipher.txt")))
               #",")
        chars (map
               #(char (Integer/parseInt %))
               ascii )
        plaintext (try-key chars "exp")
        ]
    (reduce + (map byte plaintext))
    )
  )

