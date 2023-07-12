#!/usr/bin/env python
# Set the "repos" member of a json file
#
# Copyright 2015 Colin Walters <walters@verbum.org>
# Licensed under the new-BSD license (http://www.opensource.org/licenses/bsd-license.php)

import os
import sys
import json

path = sys.argv[1]
data = json.load(open(path))
data['repos'] = sys.argv[2:]

tmppath = path + '.tmp'
with open(tmppath, 'w') as f:
    json.dump(data, f, indent=4)
os.rename(tmppath, path)
