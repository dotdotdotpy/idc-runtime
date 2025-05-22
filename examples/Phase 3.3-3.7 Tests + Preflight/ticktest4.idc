on start
  remember counter 0
  remember mode global
  respond "Ticktest4 started."

every 1
  respond "Tick {{counter}} entered"
  load counter from "({{counter}} + 1)"
  load mode from '"tick"'
  if "{{counter}} % 25 == 0"
    respond "Checkpoint at {{counter}} (mode={{mode}})"
  if "{{counter}} == 100"
    respond "COMPLETE at tick {{counter}} (mode={{mode}})"
    reflect memory all
    reflect stack
    stop
