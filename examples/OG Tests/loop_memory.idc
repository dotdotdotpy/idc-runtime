load counter from 3

while counter
    respond "Looping..."
    delete counter
    load counter from 2
    if counter
        respond "Counter still non-zero"
    else
        break

respond "Exited loop"
