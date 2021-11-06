


# Prerequisite

### Setup On Windows

```console 
pip install virtualenv && virtualenv envio
```

```console 
.\envio\Scripts\activate && pip install django djangorestframework psycopg2
```

### Setup On Linux

```console 
sudo pip install virtualenv && virtualenv envio
```

```console 
source envio/bin/activate && pip install django djangorestframework psycopg2
```

# Run Server

```console 
cd cus_dev
```

```console 
python manage.py migrate && python manage.py runserver
```

[API](http://localhost:8000/envio)

# Note

> db username and password: _envio_

> db name: _envio_