on start
  remember counter 0

every 1
  load "{{counter}} + 1" counter to global

  if {{counter}} == 3
    respond "Triggering all reflections"
    reflect memory json
    reflect flags
    reflect event
    reflect stack
    reflect log
    stop
