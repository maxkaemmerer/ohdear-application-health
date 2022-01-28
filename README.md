# ohdear-application-health

This application provides an endpoint at `GET http://localhost:8000/health` that returns OhDear application health statistics for disk, memory and cpu.

## Available Environment Variables

All environment variables have default values except for `OHDEAR_TOKEN`, make sure to provide your own.

* `OHDEAR_TOKEN=` the token used by OhDear to authenticate (make sure to provide this, otherwise this endpoint is available to everyone). OhDear passes this via the `oh-dear-health-check-secret` header
* `DISK_FAILURE_THRESHOLD=90` the percentage of total used disk space that causes a failure
* `DISK_WARNING_THRESHOLD=80` the percentage of total used disk space that causes a warning
* `MEMORY_FAILURE_THRESHOLD=80` the percentage of total used memory that causes a failure
* `MEMORY_WARNING_THRESHOLD=70` the percentage of total used memory that causes a warning
* `CPU_FAILURE_THRESHOLD=80` the percentage of total used cpu that causes a failure
* `CPU_WARNING_THRESHOLD=70` the percentage of total used cpu that causes a warning
* `CPU_TIMESPAN_MS=500` the timespan in which cpu usage is measured (in ms)


## Usage in Docker Compose
```docker
version: "3.7"

services:
  ohdear-application-health:
    image: maxkaemmerer/ohdear-application-health:1.0.0
    restart: unless-stopped
    environment:
      - OHDEAR_TOKEN=your-token
      - DISK_FAILURE_THRESHOLD=90
      - DISK_WARNING_THRESHOLD=80
      - MEMORY_FAILURE_THRESHOLD=80
      - MEMORY_WARNING_THRESHOLD=70
      - CPU_FAILURE_THRESHOLD=80
      - CPU_WARNING_THRESHOLD=70
      - CPU_TIMESPAN_MS=500
```