

# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


_________________________________________________________________
TODO : 

	https://github.com/fawkesley/pub-sub-python-zeromq-protobuf
	https://typer.tiangolo.com/

only this script can run the uniXerr protocol using zmq.
enable authentication for this file and uniXerr app server
_________________________________________________________________

'''
import zmq


context = zmq.Context()
print("Connecting to uniXerr server")
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")


for request in range(10):
	print(f"[+] Sending request {request}")
	
	# send your cnc through socket to app.py
	# ...

	socket.send(b"Hello")

	message = socket.recv()
	print(f"[+] Received reply {request}, {message}")
