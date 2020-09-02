import json
import sys
sys.path.append("D://data/txt/ruLib")
import ruPyInven

price = json.dumps([
dict(
        product="b",
        price=3,
        date="2019-01-01",
    ),
    dict(
        product="a",
        price=1,
        date="2019-01-01",
    ),
    dict(
        product="a",
        price=2,
        date="2019-01-03",
    ),
])
raw = json.dumps([
dict(
        product="b",
        qty=10,
        date="2019-01-01",
    ),
    dict(
        product="a",
        qty=10,
        date="2019-01-01",
    ),
    dict(
        product="a",
        qty=-1,
        date="2019-01-03",
    ),
])
days=json.dumps(["2019-01-01","2019-01-02","2019-01-03"])
res1=ruPyInven.inven_everyday(raw,days)
res2=ruPyInven.value_everyday(raw,days,price)
print (1111111111,res1,res2)
