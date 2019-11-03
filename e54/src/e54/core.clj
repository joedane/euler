(ns e54.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn make-hand [line]
  (->>
   (str/split line #" ")
   (partition 5)
   )
  )

(defn read-hands [file-name]
  (with-open [rdr (clojure.java.io/reader (clojure.java.io/resource file-name))]
    (loop [lines (line-seq rdr)
           hands []]
      (if (empty? lines)
        hands
        (recur (rest lines) 
               (conj hands (make-hand (first lines)))))
      )
    )
  )

(defn rank-value [rank]
  (cond
    (= rank \2) 1
    (= rank \3) 2
    (= rank \4) 3
    (= rank \5) 4
    (= rank \6) 5
    (= rank \7) 6
    (= rank \8) 7
    (= rank \9) 8    
    (= rank \T) 9
    (= rank \J) 10
    (= rank \Q) 11
    (= rank \K) 12
    (= rank \A) 13
    :else
    (throw (IllegalArgumentException. (format "invalid input: %c" rank)))
    )
  )

(defn rank-map [hand]
  "return a map the keys of which are the ranks in the hand, and the values a collection of suits"
  (reduce
   (fn [m card]
     (update-in m [(first card)] (fnil (fn [old] (str old (second card))) "")))
   {}
   hand)
  )

(defn suit-map [hand]
  "return a map the keys of which are the suits in the hand, and the values a collection of ranks"
  (reduce
   (fn [m card]
     (update-in m [(second card)] (fnil (fn [old] (str old (first card))) "")))
   {}
   hand)
  )

(defn flush? [hand]
  (let [suits (suit-map hand)]
    (if (= 1 (count (keys suits)))
      true
      nil
      )
    )
  )

(defn straight? [hand]
  (let [ranks (sort (map #(rank-value (first %)) hand))]
    (if (every?
         #(= (inc (nth ranks %)) (nth ranks (inc %)))
         (range 0 4))
      true
      nil
      )
    )
  )

(defn has-n-of-rank? [hand n] 
  (let [ranks (rank-map hand)
        rank (first
              (filter #(= n (count (ranks %))) (keys ranks)))
        ]
    (if (nil? rank) false rank)
    ;; (if-let [card-set (some (fn [rank] (= n (count (ranks rank)))) (keys ranks))] 
    ;;   card-set
    ;;   false)
    )
  )

(defn one-pair? [hand]
  (let [ranks (rank-map hand)
        maybe-pair (filter (fn [rank] (= 2 (count (ranks rank))))
                           (keys ranks))
        ]
    (if (= 1 (count maybe-pair))
      maybe-pair
      nil
      )
    )
  )

(defn two-pair? [hand]
  (let [ranks (rank-map hand)
        maybe-pairs (filter (fn [rank] (= 2 (count (ranks rank))))
                            (keys ranks))
        ]
    (if (= 2 (count maybe-pairs))
      maybe-pairs
      nil))
  )

(defn three-of-a-kind? [hand]
  (has-n-of-rank? hand 3)
  )

(defn full-house? [hand]
  (and (one-pair? hand) (three-of-a-kind? hand))
  )

(defn four-of-a-kind? [hand]
  (has-n-of-rank? hand 4)
  )

(defn straight-flush? [hand]
  (and (straight? hand) (flush? hand))
  )

(defn highest-rank [hand] 
  (first (sort (comp - compare) (map #(rank-value (first %)) hand)))
  )

(defn royal-flush? [hand]
  (and (straight-flush? hand) (= highest-rank 13))
  )


(defn highest-card [h1 h2]
  "Return 1 or 2 if h1 or h2 respectively have the highest card"
  (letfn [(highest [r1 r2]
            (if (empty? r1) (throw (IllegalArgumentException. "TIE!")))
            (if (> (first r1) (first r2))
              1
              (if (< (first r1) (first r2)) 
                2
                (recur (rest r1) (rest r2))
                ))
            )]
    (highest (sort (comp - compare) (map #(rank-value (first %)) h1))
             (sort (comp - compare) (map #(rank-value (first %)) h2)))
    )
  )

(defn tiebreak [h1 best1 h2 best2]
  "Return 1 if h1 beats h2, or 2 if otherwise"
  (cond
    (= best1 10) (throw (IllegalArgumentException. "Royal Fliush can't tie"))
    (= best1 9) (if (= (highest-rank h1) (highest-rank h2))
                  (throw (IllegalArgumentException. "Straight Flush can't tie"))
                  (if (> (highest-rank h1) (highest-rank h2))
                    1 2)
                  )
    (= best1 8) (let [ranks1 (rank-map h1) 
                      four1 (first (filter (fn [rank] (= 4 (count (ranks1 rank)))) (keys ranks1)))
                      ranks2 (rank-map h2)
                      four2 (first (filter (fn [rank] (= 4 (count (ranks2 rank)))) (keys ranks2)))
                      ]
                  (if (= (rank-value four1) (rank-value four2))
                    (throw (IllegalArgumentException. "4OAK can't tie"))
                    (if (> (rank-value four1) (rank-value four2))
                      1 2))
                  )
    (= best1 7) (if (> (rank-value (has-n-of-rank? h1 3))  ; full house
                       (rank-value (has-n-of-rank? h2 3)))
                  1
                  (if (> (rank-value (has-n-of-rank? h1 2))
                         (rank-value (has-n-of-rank? h2 2)))
                    1
                    2))
    (or (= best1 6) (= best1 5))(if (= 1 (highest-card h1 h2)) ; Flush and straight
                                  1
                                  2)
    (= best1 4) (if (> (rank-value (has-n-of-rank? h1 3))  ; Three of a kind
                       (rank-value (has-n-of-rank? h2 3))) 
                  1
                  2)
    (= best1 3)  (let [p1 (sort (comp - compare) (map #(rank-value %) (two-pair? h1)))  ; two pair
                       p2 (sort (comp - compare) (map #(rank-value %) (two-pair? h2)))  ]
                   (cond
                     (> (first p1) (first p2)) 1
                     (< (first p1) (first p2)) 2
                     (> (second p1) (second p2)) 1
                     :else 2
                     )
                   )                     
    (= best1 2) (let [p1 (one-pair? h1)
                      p2 (one-pair? h2)]
                  (if (> (rank-value (first p1))
                         (rank-value (first p2)))
                    1
                    2)
                  )
    (= best1 1)  (if (= 1 (highest-card h1 h2))
                   1
                   2)
    :else (throw (IllegalArgumentException. "????"))
    )
  )

(defn best [hand]
  (cond
    (royal-flush? hand) 10
    (straight-flush? hand) 9
    (four-of-a-kind? hand) 8
    (full-house? hand) 7
    (flush? hand) 6
    (straight? hand) 5
    (three-of-a-kind? hand) 4
    (two-pair? hand) 3
    (one-pair? hand) 2
    :else 1
    )
  )

(defn score-to-hand [score]
  (cond
    (= score 10) "Royal Flush"
    (= score 9) "Straight Flush"
    (= score 8) "Four of a Kind"
    (= score 7) "Full House"
    (= score 6) "Flush"
    (= score 5) "Straight"
    (= score 4) "Three of a Kind"
    (= score 3) "Two Pair"
    (= score 2) "One Pair"
    (= score 1) "High Card"
    :else (throw (IllegalArgumentException. "bad score"))
    )
  )

(defn winner [h1 best1 h2 best2]
  (if (> best1 best2)
    1
    (if (< best1 best2)
      2
      (tiebreak h1 best1 h2 best2)
      )
    )
  )

(defn play [hands]
  (let [[h1 h2] hands
        best1 (best h1)
        best2 (best h2)
        w (winner h1 best1 h2 best2)]
    
    (if (= best1 4) (prn (format "W%d Hand 1: [%s] %s    Hand 2: [%s] %s\n" 
                                     w
                                     (clojure.string/join " " h1) 
                                     (score-to-hand best1) 
                                     (clojure.string/join " " h2) 
                                     (score-to-hand best2))))
    
    w
    )
  )


(defn -main
  "Euler problem 54"
  [& args]
  (let [hands (read-hands "p054_poker.txt")]
    (count (filter #(= 1 %) (map play hands)))
    )
  )
