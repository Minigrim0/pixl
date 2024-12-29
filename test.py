#!/usr/bin/python3

import sys
import socket
from PIL import Image
import json



im = Image.open(sys.argv[1]).convert('RGBA')
_, _, w, h = im.getbbox()
# print('Image:', w, h)
result_list = []
for x in range(w):
    for y in range(h):
        r, g, b, a = im.getpixel((x, y))
        # pixel(x, y, r, g, b, a)
        result_list.append('PX %d %d %02x%02x%02x\n' % (x, y, r, g, b))

with open(sys.argv[2], "w+") as f:
    for i in result_list:
        f.write(i)
