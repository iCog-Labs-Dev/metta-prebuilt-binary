!(TreeNode (Value Nil False ROOT) Nil (Cons NilNode (Cons (TreeNode (Value Nil True OR) Nil (Cons (TreeNode (Value a False LITERAL) Nil Nil) (Cons (TreeNode (Value Nil True AND) Nil (Cons (TreeNode (Value b False LITERAL) Nil Nil) (Cons (TreeNode (Value c False LITERAL) Nil Nil) Nil))) Nil))) Nil)))
!(assertEqualToResult 3  (+ 1 2) )

; ; !(get-type )

; (: isConsistent (-> (List Tree) Bool))
; (= (isConsistent Nil) True)
; (= (isConsistent (Cons $x $xs))
;   (if (detectConsistency $x $xs)
;     (isConsistent $xs)
;     False
;   )
; )

; !(isConsistent 
;   (Cons 
;     (TreeNode 
;       (Value "A" True AND) 
;       Nil 
;       (Cons 
;         (TreeNode (Value "B" True OR) Nil Nil) 
;         (Cons 
;           (TreeNode (Value "C" False LITERAL) Nil Nil) 
;           Nil
;         )
;       )
;     )
;     (Cons 
;       (TreeNode (Value "B" True OR) Nil Nil) 
;       (Cons 
;         (TreeNode (Value "C" True LITERAL) Nil Nil) 
;         (Cons 
;           (TreeNode (Value "A" True LITERAL) Nil Nil) 
;           Nil
;         )
;       )
;     )
;   )
; )