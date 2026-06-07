if exists("b:current_syntax")
  finish
endif

syntax match fretwireMovedLine "^.\{-}\ze:>"
syntax match fretwireDestination ":>\zs.*$" contains=fretwireMoveMarker
syntax match fretwireMoveMarker ":>" containedin=ALL

highlight default link fretwireMovedLine Comment
highlight default link fretwireMoveMarker Keyword
highlight default link fretwireDestination String

let b:current_syntax = "fretwire"
