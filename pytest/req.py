import  requests
def test_static_dir():
    url="http://39.96.40.177:8888//static/pic/1.jpg"
    res=requests.get(url)
    print (res.text)
test_static_dir()
