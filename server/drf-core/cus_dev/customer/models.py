




from django.db import models
import uuid




class Customer(models.Model):
    timestamp = models.TimeField(auto_now_add=True)
    reading = models.FloatField()
    device_id = models.UUIDField()
    customer_id = models.UUIDField()