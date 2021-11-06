


from django.db.models.functions import TruncDate
from django.db.models import query
from django.db.models.aggregates import Avg, Count
from django.shortcuts import render
from rest_framework.utils import serializer_helpers
from .models import Customer
from rest_framework import generics
from .serializers import CustomerSerializer




class CustomerCreate(generics.CreateAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerSerializer




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
                                        .extra(select={'timestamp': "FLOOR (EXTRACT (EPOCH FROM timestamp::timestamp without time zone) / '300')"}) \
                                        .annotate(readings=Avg('reading'))
        return queryset # we have to serialize this queryset into the json using our Serializer