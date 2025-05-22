// test_combined.idc

// Trigger 'start' event
on "start"
respond "Starting periodic event loop test"

// Load memory and perform an arithmetic operation
load a from "2"
load b from "3"
respond "Sum of a and b: {{a}} + {{b}}"

// Run periodic event loop with limit
every 1s with limit 3
respond "Periodic event loop test completed"

// Cleanup memory after event
delete a
delete b
respond "Memory cleaned up. Remaining: {{a}}, {{b}}"
