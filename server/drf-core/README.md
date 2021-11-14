



# Django Rest Framework Server for BitDad LMS APIs

#### Prerequisite

### Setup On Windows

```console 
pip install virtualenv && virtualenv bitdadenv
```

```console 
.\bitdadenv\Scripts\activate && pip install -r requirements.txt
```

### Setup On Linux

```console 
sudo pip install virtualenv && virtualenv bitdadenv
```

```console 
source bitdadenv/bin/activate && pip install -r requirements.txt
```

#### Run Server

```console 
cd cus_dev
```

```console 
python manage.py migrate && python manage.py runserver
```

[APIs](http://localhost:8000/lms)

# Note

> db username and password: _lms_

> db name: _lms_