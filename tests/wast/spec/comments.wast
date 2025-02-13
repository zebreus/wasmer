;; Test comment syntax

;;comment

;;;;;;;;;;;

    ;;comment

( ;;comment
module;;comment
);;comment

;;)
;;;)
;; ;)
;; (;

(;;)

(;comment;)

(;;comment;)

(;;;comment;)

(;;;;;;;;;;;;;;)

(;(((((((((( ;)

(;)))))))))));)

(;comment";)

(;comment"";)

(;comment""";)

;; ASCII 00-1F, 7F
(;	

;)

;; null-byte followed immediately by end-of-comment delimiter
(;;)


(;Heiße Würstchen;)

(;;)

(;comment
comment;)

         	(;comment;)

(;comment;)((;comment;)
(;comment;)module(;comment;)
(;comment;))(;comment;)

(;comment(;nested;)comment;)

(;comment
(;nested
;)comment
;)

(module
  (;comment(;nested(;further;)nested;)comment;)
)

(;comment;;comment;)

(;comment;;comment
;)

(module
  (;comment;;comment(;nested;)comment;)
)


;; Newline recognition

(module quote
  "(func (export \"f1\") (result i32)"
  "  (i32.const 1)"
  "  ;; comment\0a"
  "  (return (i32.const 2))"
  "\0a"
  ")"
  "(func (export \"f2\") (result i32)"
  "  (i32.const 1)"
  "  ;; comment\0d"
  "  (return (i32.const 2))"
  "\0a"
  ")"
  "(func (export \"f3\") (result i32)"
  "  (i32.const 1)"
  "  ;; comment\0d\0a"
  "  (return (i32.const 2))"
  "\0a"
  ")"
)

(assert_return (invoke "f1") (i32.const 2))
(assert_return (invoke "f2") (i32.const 2))
(assert_return (invoke "f3") (i32.const 2))
