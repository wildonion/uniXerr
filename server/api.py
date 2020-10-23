


# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''

from cassandra.util import datetime_from_uuid1
from uuid import uuid1
from .db import init
from .db.schema import User, Position
from typing import Optional
from fastapi import FastAPI
from pydantic import BaseModel
from datetime import datetime
from pathlib import Path
import pandas as pd
import numpy as np
import pickle
import os



api = FastAPI()
db = None


@api.on_event("startup")
async def init_db():
	global db
	db = init()


@api.on_event("shutdown")
async def terminate_db():
	db.close()


@api.get("/")
async def welcome():
	return {"Welcome to": "uniXerr API Server"}


# #########------------------------------------------------------------------------------------------


@api.get("/users/info/{limit}")
async def get_users_info(limit: int):
	users_list_ = []
	users_list = []
	response_status = 200
	try: # for timeuuid ops refer to UUID and timeuuid functions on datastax docs
		future = db.query(f"select id, toTimestamp(time), rollcall_score, class_activity, discipline, total_quizzes_avg from users_info limit {limit};", [])
		users_ = future.result()
		for user_ in users_:
			users_list_.append(user_)
		users = User.objects().limit(limit)
		for user in users:
			user_dict = dict(user)
			user_dict["time"] = datetime_from_uuid1(user_dict["time"]) # convert uuid1 to datetime
			users_list.append(user_dict)
	except Exception as e:
		print(f"[Exception] ::: {e}")
		response_status = 500
	return {"db.query()_users": users_list_, "schema_users": users_list, "response_status": response_status}



# #########------------------------------------------------------------------------------------------


@api.get("/user/info/{user_id}") # ::: 'allow filtering' is only for development :::
async def get_user_info(user_id: int):
	response_status = 200
	try:
		future = db.query("select id, toTimestamp(time), rollcall_score, class_activity, discipline, total_quizzes_avg FROM users_info where id=? allow filtering", [user_id])
		user_ = future.result()[0]
		user = User.objects(id=user_id).allow_filtering()[0]
		user.time = datetime_from_uuid1(user.time)
	except Exception as e:
		print(f"[Exception] ::: {e}")
		response_status = 500
	return {"db.query()_user": user_, "schema_user": user, "response_status": response_status}


# #########------------------------------------------------------------------------------------------


@api.get("/users/positions/{limit}")
async def get_users_positions(limit: int):
	positions_list_ = []
	positions_list = []
	response_status = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions limit {limit};", [])
		positions_ = future.result()
		for position_ in positions_:
			positions_list_.append(position_)
		positions = Position.objects().limit(limit)
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"]) # convert uuid1 to datetime
			positions_list.append(position_dict)
	except Exception as e:
		response_status = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions": positions_list_, "schema_positions": positions_list, "response_status": response_status}



# #########------------------------------------------------------------------------------------------


@api.get("/users/position-latent/{position}") # ::: 'allow filtering' is only for development :::
async def get_users_position_latent(position: str):
	positions_latent_list_ = []
	positions_latent_list = []
	response_status = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_latent = ? allow filtering;", [position])
		positions_ = future.result()
		for position_ in positions_:
			positions_latent_list_.append(position_)
		positions = Position.objects.filter(position_latent=position).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_latent_list.append(position_dict)
	except Exception as e:
		response_status = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_latent": positions_latent_list_, "schema_positions_latent": positions_latent_list, "response_status": response_status}



# #########------------------------------------------------------------------------------------------


@api.get("/users/position-raw/{position}") # ::: 'allow filtering' is only for development :::
async def get_users_position_raw(position: str):
	positions_raw_list_ = []
	positions_raw_list = []
	response_status = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_raw = ? allow filtering;", [position])
		positions_ = future.result()
		for position_ in positions_:
			positions_raw_list_.append(position_)
		positions = Position.objects.filter(position_raw=position).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_raw_list.append(position_dict)
	except Exception as e:
		response_status = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_raw": positions_raw_list_, "schema_positions_raw": positions_raw_list, "response_status": response_status}



# #########------------------------------------------------------------------------------------------


@api.get("/users/positions/{latent}/{raw}") # ::: 'allow filtering' is only for development :::
async def get_users_same_positions(latent: str, raw: str):
	positions_LandR_list_ = []
	positions_LandR_list = []
	response_status = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_latent = ? and position_raw = ? allow filtering;", [latent, raw])
		positions_ = future.result()
		for position_ in positions_:
			positions_LandR_list_.append(position_)
		positions = Position.objects.filter(position_latent=latent).filter(position_raw=raw).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_LandR_list.append(position_dict)
	except Exception as e:
		response_status = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_raw": positions_LandR_list_, "schema_positions_raw": positions_LandR_list, "response_status" : response_status}



# #########------------------------------------------------------------------------------------------


@api.get("/user/positions/{user_id}") # ::: 'allow filtering' is only for development :::
async def get_user_positions(user_id: int):
	response_status = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where user_id = ? allow filtering;", [user_id])
		positions_ = future.result()[0]
		positions = Position.objects(user_id=user_id).allow_filtering()[0]
		positions.time = datetime_from_uuid1(positions.time)
	except Exception as e:
		response_status = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions": positions_, "schema_positions": positions, "response_status": response_status}


# #########------------------------------------------------------------------------------------------


class Info(BaseModel):
    user_id: int = None
    rollcall_score: int = 0
    class_activity: float = 0.0
    discipline: float = 0.0
    total_quizzes_avg: float = 0.0


@api.post('/user/classify/position') # at the time of using this route we don't have input_data.csv and classified csv files
async def predict_position(info: Info): # classify the position of a single user
	curr_dir = os.path.dirname(os.path.abspath(__file__))
	classifier_obj_path = os.path.abspath(curr_dir + f"/../core/position_classification/utils/classifier.obj") 
	response_status = 201

	if os.path.exists(classifier_obj_path):
		try:
			with open(classifier_obj_path, 'rb') as classifier_file:
				classifier_ = pickle.load(classifier_file)
			input_data = np.array([info.rollcall_score, info.class_activity, info.discipline, info.total_quizzes_avg]).reshape(1, -1)
			result = classifier_(input_data)
			position, data_type = result[0], result[1]
			user = User(id=info.user_id, time=uuid1(), rollcall_score=info.rollcall_score, 
							class_activity=info.class_activity, discipline=info.discipline, 
							total_quizzes_avg=info.total_quizzes_avg).save()

			data = {"user_id": info.user_id, "time": uuid1(), f"position_{data_type}": str(position[0])}
			user_position = Position(**data).save()
			msg = "now call /user/positions/{user_id} route to see the classification result"
		except Exception as e:
			print(f"[Exception] ::: {e}")
			response_status = 500
			msg = "classification error!"
	else:
		msg = "pre-trained model has been deleted you must train it first"
		response_status = 404 
	
	return {"response_status": response_status, "msg": msg}



# #########------------------------------------------------------------------------------------------


@api.get("/users/add/info") # add rows of users features from csv file into users_info table
async def add_users_info():
	futures = []
	can_we_move = True
	response_status = 201
	msg = "all users' features inserted successfully"
	input_data = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data.csv'
	
	if os.path.exists(input_data):
		df = pd.read_csv(input_data)
		for i in range(len(df)):
			try:
				user = User(id=df.iloc[i].user_id, time=uuid1(), rollcall_score=df.iloc[i].rollcall_score, 
							class_activity=df.iloc[i].class_activity, discipline=df.iloc[i].discipline, 
							total_quizzes_avg=df.iloc[i].total_quizzes_avg)
				user.save()



				# #### --------------------------------------------------------------------------------
				# #### if you want to use db.query just comment User model to avoid duplicate insertion
				# #### --------------------------------------------------------------------------------

				# future = db.query("insert into users_info (id, time, rollcall_score, class_activity, discipline, total_quizzes_avg) values (?, ?, ?, ?, ?, ?)", 
				# 		  	  [df.iloc[i].user_id.astype('int'), uuid1(), df.iloc[i].rollcall_score.astype('int'), 
				# 		  	   df.iloc[i].class_activity, df.iloc[i].discipline, df.iloc[i].total_quizzes_avg
				# 		  	 ])
				# futures.append(future) # do what ever you want with futures like f.result()
				


			except Exception as e:
				print(f"[Exception] ::: {e}")
				can_we_move = False
				response_status = 500
				msg = "can't insert data into db, check server!"

		if can_we_move:
			imported_time = datetime.now().strftime('%Y-%m-%d_%H-%M-%S')
			csv_must_be_in = Path(os.path.dirname(os.path.abspath(__file__))+'/db/_imported/users_info/'+imported_time)
			try:
				csv_must_be_in.mkdir(parents=True)
				imported_csv_path = os.path.dirname(os.path.abspath(__file__))+f'/db/_imported/users_info/{imported_time}/input_data.csv' 
				os.rename(input_data, imported_csv_path)
			except Exception as e:
				print(f"[Exception] ::: {e}")
				response_status = 500
				msg = "can't move the file, check server!"

	else:
		response_status = 404
		msg = "no classification on csv file has done thus we don't have input_data and classified positions csv files"

	return {"response_status": response_status, "msg": msg}


# #########------------------------------------------------------------------------------------------



@api.get("/users/add/positions") # merge classified positions and then add those to users_positions table
async def add_users_positions():
	futures = []
	response_status = 201
	msg = "all users' positions inserted successfully"
	can_we_move = True
	classified_latent = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data_classified_positions_using-pre-trained_model_on-latent.csv'
	classified_raw = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data_classified_positions_using-pre-trained_model_on-raw.csv'
	
	if os.path.exists(classified_latent):
		df_latent = pd.read_csv(classified_latent)
		position_latent = df_latent["position"]
		user_id = df_latent["user_id"]
		users_length = len(user_id)
		for i in range(users_length):
			try:
				user_position = Position(user_id=user_id.iloc[i], time=uuid1(), position_latent=position_latent.iloc[i])
				user_position.save()
				


				# #### ------------------------------------------------------------------------------------
				# #### if you want to use db.query just comment Position model to avoid duplicate insertion
				# #### ------------------------------------------------------------------------------------

				# future = db.query("insert into users_positions (user_id, time, position_latent) values (?, ?, ?)", 
				# 			  			[user_id.iloc[i], uuid1(), position_latent.iloc[i]])
				# futures.append(future) # do what ever you want with futures like f.result()



			except Exception as e:
				print(f"Exception ::: {e}")
				can_we_move = False
				response_status = 500
				msg = "can't insert data into db, check server!"

		if can_we_move:
			imported_time = datetime.now().strftime('%Y-%m-%d_%H-%M-%S')
			csv_must_be_in = Path(os.path.dirname(os.path.abspath(__file__))+'/db/_imported/users_positions/'+imported_time)

			try:
				csv_must_be_in.mkdir(parents=True)
				classified_latent_file_name = 'input_data_classified_positions_using-pre-trained_model_on-latent.csv'
				imported_classified_latent_csv_path = os.path.dirname(os.path.abspath(__file__))+f'/db/_imported/users_positions/{imported_time}/{classified_latent_file_name}'
				os.rename(classified_latent, imported_classified_latent_csv_path)
			except Exception as e:
				print(f"[Exception] ::: {e}")
				response_status = 500
				msg = "can't move the file, check server!"

	if os.path.exists(classified_raw):
		df_raw = pd.read_csv(classified_raw)
		position_raw = df_raw["position"]
		users_raw = len(user_id)
		for i in range(users_length):
			try:
				user_position = Position(user_id=user_id.iloc[i], time=uuid1(), position_raw=position_raw.iloc[i])
				user_position.save()
				


				# #### ------------------------------------------------------------------------------------
				# #### if you want to use db.query just comment Position model to avoid duplicate insertion
				# #### ------------------------------------------------------------------------------------

				# future = db.query("insert into users_positions (user_id, time, position_raw) values (?, ?, ?)", 
				# 			  				[user_id.iloc[i], uuid1(), position_raw.iloc[i]])
				# futures.append(future) # do what ever you want with futures like f.result()



			except Exception as e:
				print(f"Exception ::: {e}")
				can_we_move = False
				response_status = 500
				msg = "can't insert data into db, check server!"

		if can_we_move:
			imported_time = datetime.now().strftime('%Y-%m-%d_%H-%M-%S')
			csv_must_be_in = Path(os.path.dirname(os.path.abspath(__file__))+'/db/_imported/users_positions/'+imported_time)
			try:
				csv_must_be_in.mkdir(parents=True)				
				classified_raw_file_name = 'input_data_classified_positions_using-pre-trained_model_on-raw.csv'
				imported_classified_raw_csv_path = os.path.dirname(os.path.abspath(__file__))+f'/db/_imported/users_positions/{imported_time}/{classified_raw_file_name}' 
				os.rename(classified_raw, imported_classified_raw_csv_path)
			except Exception as e:
				print(f"[Exception] ::: {e}")
				response_status = 500
				msg = "can't move the file, check server!"

	else:
		response_status = 404
		msg = "no classification has done thus we don't have classified positions csv files"

	return {"response_status": response_status, "msg": msg}


# #########------------------------------------------------------------------------------------------