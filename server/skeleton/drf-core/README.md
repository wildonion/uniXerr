



# DRF APIs for AI Cores

### Prerequisite

#### Setup On Windows

```console 
pip install virtualenv && virtualenv corenv
```

```console 
.\corenv\Scripts\activate && pip install -r requirements.txt
```

#### Setup On Linux

```console 
sudo pip install virtualenv && virtualenv corenv
```

```console 
source corenv/bin/activate && pip install -r requirements.txt
```

### Run Server

```console 
python manage.py migrate && python manage.py runserver
```

[APIs](http://localhost:8000/ai/core)
