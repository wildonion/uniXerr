






# load pc_features_labeled.csv then train the model to predict the unseen students position

import numpy as np

n = 5
A = np.diag(list(range(1, n+1)))

diag_B = np.diag(list(range(1, n)))
col_B = np.hstack((diag_B, np.zeros((diag_B.shape[0], 1), dtype=diag_B.dtype))) # add a new col of zeros at the end of mat
B = np.insert(col_B, 0, 0, axis=0) # insert zeros at first row

diag_C = np.diag(list(range(n-1, 0, -1)))
col_C = np.insert(diag_C, 0, 0, axis=1) # insert zeros at first col
C = np.insert(col_C, n-1, 0, axis=0) # insert zeros at the last row 

mat = np.add(np.add(A, B), C)