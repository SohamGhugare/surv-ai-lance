from websocket import create_connection

print("Connecting...")
ws = create_connection("ws://0.0.0.0:8000/ws")
print("Connection Successful!")

id_ = input("Enter ID: ")
ws.send(id_)
while True:
    i = input()
    if i == "exit":
        print("Closing Connection")
        ws.close()
        break
    ws.send(i)
    print(ws.recv())
