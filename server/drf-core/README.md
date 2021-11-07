


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

[APIs](http://localhost:8000/envio)

# Note

> db username and password: _envio_

> db name: _envio_

> Example URL to Retrieve Data: ```http://localhost:8000/envio/aggregation-retrieve/?start=2021-11-07T07:51:24.716353Z&end=2021-11-07T07:52:45.889082Z&interval=3```

> Example URL to Retrieve All Devices Related to a Customer: ```http://localhost:8000/envio/customer/?id=9c36e740-2c09-424f-b027-49300ef79fb0```

> Example URL to Retrieve All Customers Related to a Device: ```http://localhost:8000/envio/device/?id=d03f169b-77e3-40ce-9fb6-f3c8dbbda887```