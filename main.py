from os import getenv
from typing import Optional

from fastapi import FastAPI, status
from influxdb_client import InfluxDBClient, Point
from influxdb_client.client.write_api import SYNCHRONOUS, WriteApi
from pydantic import BaseModel

bucket = "Event Tracker Development Bucket"


class Event(BaseModel):
    event_name: str
    user: str
    magnitude: Optional[float]


influx_client = InfluxDBClient(
    url=getenv("INFLUX_URL"),
    token=getenv("INFLUX_TOKEN"),
    timeout=getenv("INFLUX_TIMEOUT"),
    org=getenv("INFLUX_ORG"),
)
# influx_client = InfluxDBClient.from_config_file("influx.ini")
influx_writer = influx_client.write_api(write_options=SYNCHRONOUS)

app = FastAPI()


@app.get("/")
def status_msg():
    return {"message": "OK"}


@app.post("/event", status_code=status.HTTP_201_CREATED)
def new_event(event: Event):
    p = Point.from_dict({
        "measurement": "testing",
        "tags": {"user": event.user, "event_name": event.event_name},
        "fields": {"magnitude": event.magnitude},
    })
    influx_writer.write(bucket=bucket, record=p)
    return event
