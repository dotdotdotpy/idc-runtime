on start
  respond Welcome to IDC

load a 4
load b 5
respond Adding {{a}} and {{b}}

respond_event {{a}} + {{b}}

wait 2

every
  respond Ping!

route main_route
respond Current route: {{route_status}}

on end
  respond Done.