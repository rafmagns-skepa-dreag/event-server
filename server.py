from typing import Optional
from fastapi import FastAPI, status
from pydantic import BaseModel
from influxdb_client import InfluxDBClient, Point
from influxdb_client.client.write_api import SYNCHRONOUS, WriteApi

bucket = "Event Tracker Development Bucket"

class Event(BaseModel):
  event_name: str
  user: str
  magnitude: Optional[float]

influx_client = InfluxDBClient.from_config_file("influx.ini")
influx_writer = influx_client.write_api(write_options=SYNCHRONOUS)

app = FastAPI()

@app.get('/')
def status_msg():
    return {'message': 'OK'}

@app.post('/event', status_code=status.HTTP_201_CREATED)
def new_event(event: Event):
    d = {'measurement': 'testing', 'tags': {'user': event.user, 'event_name': event.event_name}, 'fields': {'magnitude': event.magnitude}}
    p = Point.from_dict(d)
    influx_writer.write(bucket=bucket, record=p)
    return event
