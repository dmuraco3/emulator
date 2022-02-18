file = open("file.obj", "w+")
hex = bytes.fromhex("3000").decode() + bytes.fromhex("1042").decode()
file.write(hex)
file.close()