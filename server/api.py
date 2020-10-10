


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
	csv_path_latent = os.path.dirname(os.path.abspath(__file__)) + '/dataset/input_data_classified_positions_using-pre-trained_model_on-latent.csv'
	df_latent = pd.read_csv(csv_path_latent)
	rec_latent = df.loc[df['user_id'] == user_id]
	
	csv_path_raw = os.path.dirname(os.path.abspath(__file__)) + '/dataset/input_data_classified_positions_using-pre-trained_model_on-raw.csv'
	df_raw = pd.read_csv(csv_path_raw)
	rec_raw = df.loc[df['user_id'] == user_id]
	
	return {"pre-trained on latent": {"position": rec_latent['position'].values[0]}, "pre-trained on raw": {"position": rec_raw['position'].values[0]}}