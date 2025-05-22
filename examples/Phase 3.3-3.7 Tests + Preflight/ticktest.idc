on start
    load counter "0"
    respond "Tick counter initialized."

every 2
    respond "Tick fired at count {{counter}}"
    load counter "{{counter}} + 2"
    respond_event "{{counter}}"

on missing
    respond "Missing event '{{event_name}}' was triggered."
