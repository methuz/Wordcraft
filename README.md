How to run on WSL

#config webBindAddress in AnkiConnect
{
  "webBindAddress": "0.0.0.0",
  "webBindPort": 8765,
  // ... other settings
}

#Find Windows IP
ip route | grep default | awk '{print $3}'

#Add filewall rule
wf.msc
