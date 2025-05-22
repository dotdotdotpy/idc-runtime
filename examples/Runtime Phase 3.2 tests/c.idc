on start
  load counter 1
  respond "Runtime initialized. Counter is {{counter}}."

  every
    respond "Loop start: counter is {{counter}}"
    load counter {{counter}} + 1
    respond_event {{counter}} + 10

  respond "Final counter: {{counter}}"
