import os
import shutil

os.makedirs("release", exist_ok=True)
shutil.copyfile("./target/release/linkmove-rs-nwg.exe", "./release/链接移动工具.exe")