from websocket import create_connection

print("Connecting...")
ws = create_connection("ws://0.0.0.0:8000/echo")
print("Connection Successful!")

ws.send("2173627")
while True:
    i = input()
    if i == "exit":
        print("Closing Connection")
        ws.close()
        break
    ws.send(i)
    print(ws.recv())
