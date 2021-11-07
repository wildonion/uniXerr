


from typing import Annotated
from django.db.models.functions import TruncDate
from django.db.models import query
from django.db.models.aggregates import Avg, Count
from django.shortcuts import render
from rest_framework.utils import serializer_helpers
from .models import Customer
from rest_framework import generics
from .serializers import CustomerSerializer, CustomerInsertSerializer, CustomerGetAllSerializer








# ===============================================
#                  INSERT DATA
# ===============================================
class CustomerCreate(generics.CreateAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerInsertSerializer



# ===============================================
#                  GET ALL DATA
# ===============================================
class CustomerListAll(generics.ListAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerGetAllSerializer


# ===============================================
#      GET ALL DEVIES RELATED TO A CUSTOMER 
# ===============================================
class CustomerListDevice(generics.ListAPIView):
    serializer_class = CustomerGetAllSerializer
    
    def get_queryset(self):
        if self.request.method == 'GET':
            # get all devices related to a customer
            customer_id = self.request.GET.get('id', None)
            if customer_id != None:
                queryset = Customer.objects.filter(customer_id=customer_id)
        return queryset


# ===============================================
#     GET ALL CUSTOMERS RELATED TO A DEVICE 
# ===============================================
class DeviceListCustomer(generics.ListAPIView):
    serializer_class = CustomerGetAllSerializer
    
    def get_queryset(self):
        if self.request.method == 'GET':
            # get all customers related to a device
            device_id = self.request.GET.get('id', None)
            if device_id != None:
                queryset = Customer.objects.filter(device_id=device_id)
        return queryset



# ===============================================
#           AGGREGATED BASED RETRIEVING 
# ===============================================
# retrieving is an aggregated form over time
class CustomerList(generics.ListAPIView):
    serializer_class = CustomerSerializer

    def get_queryset(self):
        if self.request.method == 'GET':
            start_time = self.request.GET.get('start', None) #-- this is the oldest time in db - inclusive
            end_time = self.request.GET.get('end', None) #-- this is the newest time in db - inclusive
            interval = self.request.GET.get('interval', 5) #-- in minutes - default is 5
            #-- reading is aggregated by averaging over time which the default is 5 minutes - reading data and average them every 5 minutes
            queryset = Customer.objects.filter(timestamp__range=(start_time, end_time)) \
                                        .extra(select={'timestamp': f"FLOOR (EXTRACT (EPOCH FROM timestamp::timestamp without time zone) / '{interval*60}')"}) \
                                        .annotate(readings=Avg('reading')) \
                                        .annotate(readings=Count('reading'))
        return queryset # we have to serialize this queryset into the json using our Serializer


