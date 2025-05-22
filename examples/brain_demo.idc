on start
  remember boot_phase "ready"
  remember status_cycle_1 "cold"
  remember status_cycle_2 "warm"
  remember status_cycle_3 "hot"
  remember system_state "idle"
  remember ping_state "silent"

every 3
  if true
    remember system_state "processing"
    remember ping_state "ping"

every 6
  if true
    remember system_state "stable"
    remember ping_state "pong"
