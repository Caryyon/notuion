# Notuion

Notuion is a Notion based TUI editor. It aims to allow you to work with the basic "blocks" that Notion provides through their API but in your wonderful
terminal instead of some GUI desktop app or web app.

** This repo is in early Alpha **

### Goals

- Make an easy to use, TUI that can represent the data and editable content of the Notion api.
- Easy to setup or Initialize
- Keeps local configuration of Notion projects

### Local Config
look at using something for local configs.
`confy` is what is suggested but gosh i hate 
rust docs. None of the examples work directly
and the docs seem different depending on the 
site you visit.

### Local Server
Will need to hit the Notion API so maybe a local
server to massage data and keep things from having
to constantly call Notion.

### TUI
seems like there is a main `tui` rust lib.
Probably just use that. Seems like there are
a few database solutions for the tui??

### stdio
get something decent for a way to handle user
input because it will be mostly what folks
will be doing.

Also think about finding away to add some `vi`
like controls and movement.
