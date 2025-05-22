on start
respond "Initializing..."
load "config" "settings.json"
if true
    respond "Configuration loaded."
else
    respond "Failed to load config."
create "new resource"
while true
    respond "Looping..."
    break
delete "resource"
