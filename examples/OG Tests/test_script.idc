on start -> {
    respond "Initial";
}

on unknown_event -> {
    respond "Error: Unknown event type.";
}

on sum -> {
    respond "Sum";
}

on product -> {
    respond "Product";
}

every 1s for 5 -> {
    respond "Event triggered every 1s";
}

wait 2s -> {
    respond "Waiting for 2s";
}

route main_route -> {
    respond "Route triggered";
}

on end -> {
    respond "End";
}
