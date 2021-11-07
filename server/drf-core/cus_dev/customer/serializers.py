



from django.db.models import fields
from rest_framework import serializers
from .models import Customer


# ================================================================================
# -- deserializing ut8 bytes coming from the socket to the Customer model
# -- serializing from Customer model back into the utf8 to send through the socket
# ================================================================================

class ReadingsSerializer(serializers.RelatedField):
    def to_representation(self, value):
        return {"timestamp": value.timestamp, "reading": value.reading}


class CustomerSerializer(serializers.ModelSerializer):
    readings = ReadingsSerializer(many=True, read_only=True)
    class Meta:
        model = Customer
        fields = ['device_id', 'customer_id', 'readings']


class CustomerInsertSerializer(serializers.ModelSerializer):
    class Meta:
        model = Customer
        fields = ['device_id', 'customer_id', 'reading']

class CustomerGetAllSerializer(serializers.ModelSerializer):
    class Meta:
        model = Customer
        fields = ['pk', 'timestamp', 'device_id', 'customer_id', 'reading']