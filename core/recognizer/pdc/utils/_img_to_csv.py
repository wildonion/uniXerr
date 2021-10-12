


# USAGE : python _img_to_csv.py --path /path/to/dataset --image-size 64

import os, sys, numpy as np
from PIL import Image
import pandas as pd
import argparse


parser = argparse.ArgumentParser(description='Image 2 CSV')
parser.add_argument('--path', action='store', type=str, help='Path to dataset')
parser.add_argument('--image-size', action='store', type=int, help='Image size to reduce')
args = parser.parse_args()

assert(60<=args.image_size<=90), "❌ Image Size Must Be Between =60 and =90 pixels"

if not os.path.exists(args.path): print("[❌] No Dataset Found!"); sys.exit(1)


dataset = {}
dataset["train_x"] = []
dataset["train_y"] = []
dataset["test_x"] = []
dataset["test_y"] = []


for dirpath, dirnames, files in os.walk(args.path):
	for filename in files:
		if filename[-2:] == "db":
			continue
		path = os.path.join(dirpath, filename)
		image = Image.open(path)
		image = image.resize((args.image_size, args.image_size)).convert('L') # convert image to grayscale
		image = np.asarray(image)
		label = int(dirpath[-1]) if dirpath[-2] == " " else int(dirpath[-2:])
		if dirpath[-14:-9] or dirpath[-13:-8] == "Train":
			print(f"✅ putting {filename} on training set with label {label}")
			dataset["train_x"].append(image)
			dataset["train_y"].append([label])
		if dirpath[-13:-9] or dirpath[-12:-8] == "Test":
			print(f"✅ putting {filename} on testing set with label {label}")
			dataset["test_x"].append(image)
			dataset["test_y"].append([label])
		else:
			continue


dataset["train_x"] = np.asarray(dataset["train_x"]).reshape(-1, args.image_size*args.image_size)
dataset["train_y"] = np.asarray(dataset["train_y"])
dataset["test_x"] = np.asarray(dataset["test_x"]).reshape(-1, args.image_size*args.image_size)
dataset["test_y"] = np.asarray(dataset["test_y"])


curr_dir = os.path.dirname(os.path.abspath(__file__))
csv_path = os.path.abspath(curr_dir + "/../dataset")

pd.DataFrame(dataset["train_x"]).to_csv(f"{csv_path}/train_x.csv", header=None, index=None)
pd.DataFrame(dataset["train_y"]).to_csv(f"{csv_path}/train_y.csv", header=None, index=None)
pd.DataFrame(dataset["test_x"]).to_csv(f"{csv_path}/test_x.csv", header=None, index=None)
pd.DataFrame(dataset["test_y"]).to_csv(f"{csv_path}/test_y.csv", header=None, index=None)
