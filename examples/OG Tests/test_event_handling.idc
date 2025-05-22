// test_event_handling.idc

// Trigger 'start' event
on "start"
respond "Start event triggered"

// Trigger 'every' event loop
every 1s with limit 3
respond "Periodic event triggered every second"

// Route event handling
route "main_route"
respond "Route event triggered: main_route"
