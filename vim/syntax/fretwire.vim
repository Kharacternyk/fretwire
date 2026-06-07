if exists("b:current_syntax")
  finish
endif

syntax match fretwireMovedLine "^.\{-}\ze\s*:>\s*\S"
syntax match fretwireDeletedLine "^.\{-}\ze\s*:>\s*$"
syntax match fretwireDestination ":>\s*\zs.*$" contains=fretwireMoveMarker
syntax match fretwireMoveMarker ":>" containedin=ALL

highlight default link fretwireMovedLine DiagnosticSignWarn
highlight default link fretwireDeletedLine DiagnosticSignError
highlight default link fretwireMoveMarker Keyword
highlight default link fretwireDestination Constant

let b:current_syntax = "fretwire"
