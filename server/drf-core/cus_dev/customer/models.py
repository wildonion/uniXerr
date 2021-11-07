




from django.db import models
from django.utils import timezone



class Customer(models.Model):
    timestamp = models.DateTimeField(auto_now=True)
    reading = models.FloatField()
    device_id = models.UUIDField()
    customer_id = models.UUIDField()