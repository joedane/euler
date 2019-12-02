(ns euler-lib.cf)

(defn make-step [f add-part div-part]
  (list f add-part div-part)
  )

(defn f-part [s]
  (first s)
  )

(defn add-part [s] 
  (second s)
  )

(defn div-part [s]
  (nth s 2)
  )

(defn stop? [s steps]
  (if (not (empty? steps)) 
    (let [f (first steps)]
      (and (= (div-part s) 1)
           (= (f-part f) (add-part s))) 
      )
    )
  )

(defn get-next-step [n this-step]
  (let [next-f (int (/ (+ (Math/sqrt n) (add-part this-step)) (div-part this-step)))
        next-add (- (* next-f (div-part this-step)) (add-part this-step))
        ]
    (make-step 
     next-f
     next-add
     (quot (- n (* next-add next-add)) (div-part this-step)))
    )
  )

(defn fraction-step 
  ([n]
   (let [first-step (get-next-step n (make-step 0 0 1))]
     (if (zero? (div-part first-step))
       (list (f-part first-step))
       (fraction-step n first-step [first-step]))
     )
   )
  ([n this-step steps]
   (if (stop? this-step steps)
     (map first (conj steps (make-step (* 2 (add-part this-step)) 0 0)))
     (let [next-step (get-next-step n this-step)]
       (fraction-step n next-step (conj steps next-step))
       )
     )
   )
  )

(defn get-continued-fraction-for-sqrt [n]
  (fraction-step n)
  )

(defn continued-fraction-digits 
  ([cf]
   (cons (first cf) (lazy-seq (continued-fraction-digits cf 1)))
   )
  ([cf n]
   (cons (nth cf n) 
         (lazy-seq 
          (continued-fraction-digits cf (let [next-n (inc n)] 
                                          (if (= next-n (count cf))
                                            1
                                            next-n)))))
   )
  )

(defn cf->convergents 
  ([cf] (let [integer-term [(first cf) 1]]
          (cons integer-term 
                (lazy-seq 
                 (cf->convergents 
                  (rest cf) 
                  [1N 0N] 
                  integer-term)))))
  ([cf-digits a b]
   (let [c (first cf-digits)
         next-convergent [(+ (* c (first b)) (first a))
                          (+ (* c (second b)) (second a))]
         ]
     (cons next-convergent
           (lazy-seq (cf->convergents (rest cf-digits) b next-convergent ))))
   
   )
  )
