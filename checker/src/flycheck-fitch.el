(require 'flycheck)

(flycheck-define-checker
 fitch-checker
 "Defines a checker for Fitch proofs using FitchVIZIER"
 :command ("/Users/dan/projects/fitch-proof/checker/target/debug/checker"
           source)
 :modes (text-mode)
 :error-patterns
 ((error line-start
         "Fatal error: parser failure near line "
         line
         ": "
         (message)
         line-end)
  (error line-start
         "Line "
         line
         ": "
         (message)
         line-end))
 :error-filter flycheck-sanitize-errors
 :predicate (lambda () (flycheck-buffer-saved-p)))


;;;###autoload
(defun flycheck-fitch-setup ()
  "Setup Flycheck for FitchVIZIER."
  (add-to-list 'flycheck-checkers 'fitch-checker))

(provide 'flycheck-fitch)
;; (flycheck-fitch-setup)
