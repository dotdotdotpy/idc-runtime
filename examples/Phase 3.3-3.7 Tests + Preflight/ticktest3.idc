set promote_always true

on start
  remember counter

every 1
  respond "Tick {{counter}} entered"
  load counter "( {{counter}} + 1 )"
  if "{{counter}} % 25 == 0"
    respond "Checkpoint hit at tick {{counter}}"
  reflect memory all
  if "{{counter}} == 100"
    respond "Final tick reached: {{counter}}"
    stop
