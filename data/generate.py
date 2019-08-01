import os
import random
import string

NUM_COLS = 10
COL_NAME_LENGTH = 5
NUM_UNIQUE_ROWS = 10000
NUM_COPIES = 100
HUGE = "huge.csv"

MANY = "many"

def name():
    return ''.join(random.choice(string.ascii_lowercase) for _ in range(COL_NAME_LENGTH))

def mean():
    return random.uniform(-100, 100)
def sd():
    return random.uniform(0, 100)

def line(vals): return ",".join(vals) + "\n"

def create_data():
    columns = line(name() for _ in range(NUM_COLS))

    params = [(mean(), sd()) for _ in range(NUM_COLS)]

    data = "".join(
        line(str(random.gauss(mu, sigma)) for mu, sigma in params)
        for _ in range(NUM_UNIQUE_ROWS)
    )

    return columns, data
    
def huge():
    columns, data = create_data()

    with open(HUGE, "w") as out:
        out.write(columns)
        for _ in range(NUM_COPIES):
            out.write(data)

def many():
    try:
        os.mkdir(MANY)
    except os.error:
        pass

    columns, data = create_data()

    for i in range(NUM_COPIES):
        with open("{}/{:02}.csv".format(MANY, i), "w") as out:
            out.write(columns)
            out.write(data)

if __name__ == '__main__':
    random.seed(1)
    huge()
    many()
