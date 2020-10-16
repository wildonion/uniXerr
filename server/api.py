


# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''

from .db import init
from .db.schema import User
from typing import Optional
from fastapi import FastAPI
from pydantic import BaseModel # you can use this with cassandra UserType
import pandas as pd
import os


api = FastAPI()
db = init()


# root of the api
@api.get("/")
async def read_root():
	return {"Welcome to": "uniXerr"}



@api.get("/users/")
async def get_users():
	users_list_ = []
	users_list = []
	future = db.query("select * from users;", []) # using db.query() method
	users_ = future.result()
	for user_ in users_:
		users_list_.append(user_)
	users = User.objects.all() # using User class model
	for user in users:
		users_list.append(dict(user))
	return {"db.query()_users": users_list_, "schema_users": users_list}



@api.get("/user/{user_id}")
async def get_user():
	future = db.query("SELECT * FROM users_info WHERE id=?", [user_id]) # using db.query() method
	user_ = future.result()[0]
	user = User.objects(_id=user_id)[0] # using User class model
	return {"db.query()_user": user_, "schema_user": user}



# TODO : add user api
# https://docs.datastax.com/en/developer/python-driver/3.24/api/cassandra/cqlengine/query/
# create a user using db.query() and User schema
# user = User(_id=19207, rollcall_score=10, class_activity=13.66, discipline=5.99, total_quizzes_avg=6.16).save()
# user = User.create(_id=19207, rollcall_score=10, class_activity=13.66, discipline=5.99, total_quizzes_avg=6.16)
# ...
