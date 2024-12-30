"""
Split the image into pixels writing commands for pixelflut and write them to a file
"""
#!/usr/bin/python3

import sys
from PIL import Image

im = Image.open(sys.argv[1]).convert('RGBA')
_, _, w, h = im.getbbox()

result_list = []
for x in range(w):
    for y in range(h):
        r, g, b, a = im.getpixel((x, y))

        if a == 0:  # Skip transparent pixels to save bandwidth
            continue
        result_list.append('PX %d %d %02x%02x%02x\n' % (x, y, r, g, b))

with open(sys.argv[2], "w+") as f:
    for i in result_list:
        f.write(i)
