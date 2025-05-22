on start
  respond "Main script starting"
  load count "5"
  trigger math_event
  respond "Main script ending"

on math_event
  respond "Triggering math logic..."
  load result "( {{count}} + 3 ) * 2"
  respond_event "{{result}}"
  if "{{result}} > 15"
    respond "Result is large"
  unless "{{result}} > 15"
    respond "Result is small"
