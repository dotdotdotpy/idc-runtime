on start
  remember counter
  respond "Initialized counter at {{counter}}"

every 2
  load "{{counter}} + 1" as counter to global
  if "{{counter}} % 4 == 0"
    respond "Counter divisible by 4: {{counter}}"
  if "{{counter}} == 10"
    stop
