i = 0xDEADBEEF
with open("file.v", "wb") as file:
    file.write(i.to_bytes(4, "big"))
