" Language: Zonkey
" Maintainer: Sam Bowden

if exists("b:current_syntax")
  finish
endif

syntax match Comment /#.*/
syntax match Identifier '\v[[:alpha:]_]+'
syntax match Function '\v[[:alpha:]_]+\ze(\s?\()'
syntax region String start=/"/ end=/"/
syntax match Number '\d\+'
syntax keyword Keyword class start function let return break continue method constructor
syntax keyword Repeat for while loop
syntax keyword Boolean true false
syntax keyword Conditional if else
syntax keyword Type Integer Float String Boolean
