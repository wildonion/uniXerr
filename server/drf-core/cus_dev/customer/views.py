



from django.db.models import query
from django.shortcuts import render
from rest_framework.utils import serializer_helpers
from .models import Customer
from rest_framework import generics
from .serializers import CustomerSerializer




class CustomerCreate(generics.CreateAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerSerializer


class CustomerList(generics.ListAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerSerializer


class CustomerDetail(generics.RetrieveAPIView):
    queryset = Customer.objects.all()
    serializer_class = CustomerSerializer