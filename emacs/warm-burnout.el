;;; warm-burnout.el --- Shared palette and utilities for Warm Burnout themes -*- lexical-binding: t; -*-

;; Author: Felipe Lima, Chris Rose
;; URL: https://github.com/felipefdl/warm-burnout
;; Version: 1.0.0
;; Package-Requires: ((emacs "29.1"))
;; Keywords: faces themes

;; This file is NOT part of GNU Emacs.

;; MIT License. See LICENSE in the repository root.

;;; Commentary:

;; Shared palette definitions and doom-themes integration for Warm Burnout.
;; A warm color theme that minimizes blue-spectrum emission. Your eyes deserved this.

;;; Code:

(defgroup warm-burnout nil
  "Warm Burnout theme options."
  :group 'faces)

(defvar warm-burnout-dark-palette
  '((bg           . "#1a1510")
    (bg-dim       . "#14120f")
    (bg-float     . "#1f1d17")
    (bg-highlight . "#222018")
    (bg-visual    . "#8aa8b840")
    (bg-search    . "#4c4126")

    (fg           . "#bfbdb6")
    (fg-dim       . "#ada69c")
    (fg-gutter    . "#a59f96")

    (comment      . "#b4a89c")
    (cursor       . "#f5c56e")
    (accent       . "#b8522e")

    (keyword      . "#ff8f40")
    (func         . "#ffb454")
    (string       . "#b4bc78")
    (type         . "#90aec0")
    (operator     . "#f29668")
    (number       . "#d4a8b8")
    (constant     . "#d4a8b8")
    (tag          . "#dc9e92")
    (property     . "#deb074")
    (member       . "#ec9878")
    (regex        . "#96b898")
    (decorator    . "#e6c08a")

    (error        . "#f49090")
    (warn         . "#b8522e")
    (info         . "#90aec0")
    (hint         . "#b4a89c")

    (added        . "#70bf56")
    (modified     . "#73b8ff")
    (deleted      . "#f26d78")
    (diff-add     . "#70bf561f")
    (diff-change  . "#73b8ff1f")
    (diff-delete  . "#f26d781f")

    (border       . "#2a2520")
    (bracket1     . "#e6c08a")
    (bracket2     . "#90aec0")
    (bracket3     . "#d4a8b8")
    (bracket4     . "#b4bc78")
    (bracket5     . "#f29668")
    (bracket6     . "#96b898"))
  "Dark variant palette for Warm Burnout.")

(defvar warm-burnout-light-palette
  '((bg           . "#F5EDE0")
    (bg-dim       . "#EDE6DA")
    (bg-float     . "#F0E8DC")
    (bg-highlight . "#E8E0D4")
    (bg-visual    . "#8aa8b840")
    (bg-search    . "#E0C890")

    (fg           . "#3a3630")
    (fg-dim       . "#5c5750")
    (fg-gutter    . "#8a8070")

    (comment      . "#544c40")
    (cursor       . "#8a6600")
    (accent       . "#b8522e")

    (keyword      . "#924800")
    (func         . "#855700")
    (string       . "#4d5c1a")
    (type         . "#285464")
    (operator     . "#8f4418")
    (number       . "#7e4060")
    (constant     . "#7e4060")
    (tag          . "#8e4632")
    (property     . "#74501c")
    (member       . "#883850")
    (regex        . "#286a48")
    (decorator    . "#7a5a1c")

    (error        . "#b03434")
    (warn         . "#b8522e")
    (info         . "#285464")
    (hint         . "#544c40")

    (added        . "#226414")
    (modified     . "#2868a0")
    (deleted      . "#c43040")
    (diff-add     . "#2264142a")
    (diff-change  . "#2868a02a")
    (diff-delete  . "#c430402a")

    (border       . "#DDD6CA")
    (bracket1     . "#7a5a1c")
    (bracket2     . "#285464")
    (bracket3     . "#7e4060")
    (bracket4     . "#4d5c1a")
    (bracket5     . "#8f4418")
    (bracket6     . "#286a48"))
  "Light variant palette for Warm Burnout.")

(defun warm-burnout--get (palette key)
  "Get color KEY from PALETTE alist."
  (cdr (assq key palette)))

(defmacro warm-burnout--with-palette (palette &rest body)
  "Bind all palette colors from PALETTE and execute BODY.
Each key in the palette becomes a local variable."
  (declare (indent 1))
  `(let ,(mapcar (lambda (entry)
                   (list (car entry) (cdr entry)))
                 (symbol-value palette))
     ,@body))

;; Doom themes integration - called after theme is loaded
(defun warm-burnout--doom-integrate (theme)
  "Apply doom-themes face overrides for THEME if doom-themes is available."
  (when (and (fboundp 'doom-themes-set-faces)
             (memq theme '(warm-burnout-dark warm-burnout-light)))
    (pcase theme
      ('warm-burnout-dark
       (doom-themes-set-faces 'warm-burnout-dark
         '(font-lock-keyword-face :foreground "#ff8f40" :weight bold)
         '(font-lock-function-name-face :foreground "#ffb454")
         '(font-lock-type-face :foreground "#90aec0" :slant italic)
         '(font-lock-string-face :foreground "#b4bc78")
         '(font-lock-comment-face :foreground "#b4a89c" :slant italic)
         '(font-lock-constant-face :foreground "#d4a8b8")))
      ('warm-burnout-light
       (doom-themes-set-faces 'warm-burnout-light
         '(font-lock-keyword-face :foreground "#924800" :weight bold)
         '(font-lock-function-name-face :foreground "#855700")
         '(font-lock-type-face :foreground "#285464" :slant italic)
         '(font-lock-string-face :foreground "#4d5c1a")
         '(font-lock-comment-face :foreground "#544c40" :slant italic)
         '(font-lock-constant-face :foreground "#7e4060"))))))

(add-hook 'enable-theme-functions #'warm-burnout--doom-integrate)

(provide 'warm-burnout)
;;; warm-burnout.el ends here
