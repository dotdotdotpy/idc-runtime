on start
  respond "Booting..."
  load a 3
  load b 4
  load result "( {{a}} + {{b}} ) / 2"
  respond_event "{{result}}"
  if "{{result}} > 3"
    respond "Above threshold."
  unless "{{result}} > 5"
    respond "Below ceiling."

on math_check
  respond "Math mode active."
