// test_memory_handling.idc

// Load values into memory
load a from "2"
load b from "3"

// Respond with values
respond "Loaded values: a = {{a}}, b = {{b}}"

// Perform arithmetic operation
respond "Sum of a and b: {{a}} + {{b}}"

// Cleanup memory
delete a
delete b
respond "Memory cleaned up. Remaining: {{a}}, {{b}}"
