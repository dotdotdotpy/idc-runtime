on start
  respond "Testing infinite loop trigger..."
  trigger loop_a

on loop_a
  respond "Inside loop_a"
  trigger loop_a
