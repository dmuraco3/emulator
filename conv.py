file = open("file.obj", "w+")
hex = bytes.fromhex("3000").decode() + bytes.fromhex("1440").decode()
file.write(hex)
file.close()