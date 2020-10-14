


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
from .db.schema import Users, Positions
from fastapi import FastAPI
import pandas as pd
import os


api = FastAPI()
db = init()
user_schema = Users(db.session)
position_schema = Positions(db.session)


# root of the api
@api.get("/")
async def read_root():
	return {"Welcome to": "uniXerr"}


# credit-info api
@api.get("/credit-info/{user_id}")
async def get_position(user_id: int):
	# TODO : first insert all classified input data positions into users table with columns position_latent and position_raw
	# 		 then fetch a classified student with his/her feature from the table
	# ...
	users = user_schema.get()
	for user in users:
		print(user)
	return {"rollcall_score": user.rollcall_score, "class_activity": user.class_activity, 
			"discipline": user.discipline, "total_quizzes_avg": user.total_quizzes_avg, 
			"positions": {"latent": user.position_latent, "raw": user.position_raw}}
		  