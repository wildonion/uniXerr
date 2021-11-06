




from django.urls import include, path
from .views import CustomerCreate, CustomerList


urlpatterns = [
    path('insert/', CustomerCreate.as_view()),
    path('data/', CustomerList.as_view()),
]