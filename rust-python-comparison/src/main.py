# Imports
import time
import numpy as np
import pandas as pd

# Decorator to time the function
def time_it(func):
    def wrapper_function(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"Python: {func.__name__} took {np.round((end - start)*1000, 2)} milliseconds")
        return result
    return wrapper_function

##### Functions #####
@time_it
def count_to_billion():
    count = 0
    for i in range(1000000000):
        count += 1
    
@time_it    
def load_airport_data():
    data = pd.read_csv('../data/airport_data.csv')
    return data
    
##### Main Function #####
if __name__ == "__main__":
    count_to_billion()
    data = load_airport_data()