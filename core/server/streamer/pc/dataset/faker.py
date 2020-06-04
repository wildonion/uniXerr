












import pandas as pd
import random
import os

def generate(samples=10000):
	secure_random_float = random.SystemRandom()
	secure_random_int = random.SystemRandom()

	dataset = {
			   'user_id': [], 'rollcall_score': [], 
			   'class_activity': [], 'discipline': [], 
			   'total_quizzes_avg': []
			   }



	for i in range(samples):
		dataset['user_id'].append(secure_random_int.randint(1, 20000))
		dataset['rollcall_score'].append(secure_random_int.randint(5, 15))
		dataset['class_activity'].append(round(secure_random_float.uniform(5.0, 15.0), 2))
		dataset['discipline'].append(round(secure_random_float.uniform(5.0, 15.0), 2))
		dataset['total_quizzes_avg'].append(round(secure_random_float.uniform(5.0, 15.0), 2))


	df = pd.DataFrame.from_dict(dataset, orient="columns")
	df.to_csv(os.path.dirname(os.path.abspath(__file__))+"/pc_features.csv", index=False)