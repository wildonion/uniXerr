



from django.db.models import fields
from rest_framework import serializers
from .models import Customer


# ================================================================================
# -- deserializing ut8 bytes coming from the socket to the Customer model
# -- serializing from Customer model back into the utf8 to send through the socket
# ================================================================================
class CustomerSerializer(serializers.ModelSerializer):
    class Meta:
        model = Customer
        fields = ['pk', 'timestamp', 'reading', 'device_id', 'customer_id']