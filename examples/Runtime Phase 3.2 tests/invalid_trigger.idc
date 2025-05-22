on start
  respond "Testing invalid trigger now..."
  trigger does_not_exist

on missing
  respond "Fallback triggered for missing event: {{event_name}}"
