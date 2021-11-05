




from django.urls import include, path
from .views import CustomerCreate, CustomerList, CustomerDetail


urlpatterns = [
    path('create/', CustomerCreate.as_view(), name='create-customer'),
    path('', CustomerList.as_view()),
    path('<int:pk>/', CustomerDetail.as_view(), name='retrieve-customer'),
]