import os

os.makedirs(r"C:\link_move_flag", exist_ok=True)

data = bytes([0] * (1024 ** 3))

for i in range(5):
    with open(fr"C:\link_move_flag\flag{i}", "wb") as file:
        file.write(data)