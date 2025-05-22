on start
  load a 5
  trigger first

on first
  respond "Triggered first. a={{a}}"
  load b "{{a}} + 1"
  trigger second

on second
  respond "In second. b={{b}} (should be 6)"
  load c "{{b}} * 2"
  trigger third

on third
  respond "Reached third. c={{c}} (should be 12)"
