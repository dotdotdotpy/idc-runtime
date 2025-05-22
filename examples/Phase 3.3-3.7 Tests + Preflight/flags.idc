set promote_always true

on start
  remember counter

every 1
  respond "Tick {{counter}} entered"
  load counter "({{counter}} + 1)"
  reflect memory all
