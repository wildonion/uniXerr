# Generated by Django 3.2.9 on 2021-11-06 19:56

from django.db import migrations, models
import django.utils.timezone


class Migration(migrations.Migration):

    dependencies = [
        ('customer', '0002_alter_customer_timestamp'),
    ]

    operations = [
        migrations.AlterField(
            model_name='customer',
            name='timestamp',
            field=models.DateTimeField(default=django.utils.timezone.now),
        ),
    ]