on start
  respond "System starting..."
  load cycles 1

every 2
  load cycles "{{cycles}} + 1"
  respond "Tick {{cycles}}: even cycle"

every 3
  respond "Tick {{cycles}}: multiple of 3"

every 5
  if {{cycles}} > 5
    respond "Tick {{cycles}} is now above 5"
