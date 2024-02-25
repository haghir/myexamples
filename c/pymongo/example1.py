#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from pymongo import MongoClient

client = MongoClient(host = 'localhost',
                     port = 27017,
                     username = 'mydbuser',
                     password = 'mydbuser',
                     authSource = 'mydb')
db = client['mydb']
col1 = db['col1']
id = col1.insert_one({"foo": "Foo",
                      "bar": "Bar"}).inserted_id
print(id)
