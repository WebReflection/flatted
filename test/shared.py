# ⚠️ RUN node test/shared.cjs FIRST
# then python test/shared.py

import os, sys

__dir__ = os.path.dirname(__file__)
sys.path.append(os.path.join(__dir__, '..', 'python'))

from flatted import parse, stringify

with open(os.path.join(__dir__, 'shared.json'), 'r') as f:
  str = f.read()

arr, obj = parse(str)

for i in range(64):
  arr = arr[0]
  obj = obj['obj']

assert len(arr) == 2 and arr[0] == 'arr' and arr[1] == 1 and obj['obj'] == 2

assert stringify(parse(str), separators=(',', ':')) == str
