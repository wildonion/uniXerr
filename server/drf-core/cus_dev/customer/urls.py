




from django.urls import include, path
from .views import CustomerCreate, CustomerList, CustomerListAll, CustomerListDevice, DeviceListCustomer


urlpatterns = [
    path('insert/', CustomerCreate.as_view()),
    path('data/', CustomerListAll.as_view()),
    path('aggregation-retrieve/', CustomerList.as_view()),
    path('customer/', CustomerListDevice.as_view()), # get all devices related to a customer
    path('device/', DeviceListCustomer.as_view()), # get all customers related to a device
]