


# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██



TODO : create a csv api of all students features from database 
		and save it in dataset folder as pc_features.csv,
		first load the input_data_classified.csv into database
		as positions document -> |user_id|position|


TODO : db query for fetching all info about
		students positions and marks.


TODO : use mongodb and graphql for req and res ops, use DevOps tools

'''


from fastapi import FastAPI
import pandas as pd
import os


api = FastAPI()



# root of the api
@api.get("/")
async def read_root():
	return {"Welcome to": "uniXerr"}


@api.get("/credit-info/{user_id}")
async def get_position(user_id: int):
	# TODO : fetch the position from db based on user_id
	csv_path = os.path.dirname(os.path.abspath(__file__)) + '/dataset/input_data_classified_positions.csv'
	df = pd.read_csv(csv_path)
	rec = df.loc[df['user_id'] == user_id]
	return {"position": rec['position'].values[0]}