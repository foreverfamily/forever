#!/usr/bin/env python
# -*- coding: utf-8 -*-
# @Time    : 18-3-21 下午10:12
# @Author  : gonghuihui
# @File    : forever.py
import urllib
from urllib import request

if __name__ == "__main__":

    url = "http://127.0.0.1:3000"
    response = urllib.request.urlopen(url)
    str = response.read().decode()
    print(str)