



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




# https://stackoverflow.com/questions/34589074/data-between-start-time-stamp-and-end-time-stamp-at-5-minutes-interval-in-postgr
# retrieving is an aggregated form over time
class CustomerList(generics.ListAPIView):
    serializer_class = CustomerSerializer

    def get_queryset(self):
        if self.request.method == 'GET':
            start_time = self.request.GET.get('start', None) #-- this is the oldest time in db - inclusive
            end_time = self.request.GET.get('end', None) #-- this is the newest time in db - inclusive
            agg_size = self.request.GET.get('agg', 5) #-- in minutes - default is 5
            # reading is aggregated by averaging over time
            queryset = Customer.objects.filter(timestamp__range=(start_time, end_time)) \
                                        .extra(select={'timestamp': "FLOOR (EXTRACT (EPOCH FROM timestamp::timestamp without time zone) / '900')"}) \
                                        .values('timestamp') \
                                        .annotate(readings=Avg('reading'))
        return queryset