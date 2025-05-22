// Start event
on start -> {
    respond "Start event triggered";
}

// Respond event
on respond_event -> {
    respond "This is a custom response event";
}

// Periodic event every 1 second for 3 iterations
every 1s for 3 -> {
    respond "Periodic event triggered";
}

// Wait event
wait 2s -> {
    respond "Wait period completed";
}

// Route event
route main_route -> {
    respond "Routing to the main route";
}

// End event
on end -> {
    respond "End event triggered";
}
