on start
  remember counter 0
  remember total 0
  respond "Program started at tick 0"

every 2
  load "{{counter}} + 1" as counter
  load "{{total}} + 2" as total
  respond_event "{{counter}}"
  if "{{counter}} >= 3"
    respond "Checkpoint reached at tick {{counter}}"
  end
  reflect memory json
end

on missing
  respond "Missing event: {{event_name}}"
