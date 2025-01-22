#!/bin/bash

IMAGE_NAME="myrabbitmq"

case "$1" in
    build)
        echo "Building RabbitMQ Docker image..."
        docker build -t $IMAGE_NAME .
        ;;
    start)
        echo "Starting RabbitMQ container..."
        docker run -d --rm --name rabbitmq -p 5672:5672 -p 15672:15672 $IMAGE_NAME
        ;;
    stop)
        echo "Stopping RabbitMQ container..."
        docker stop rabbitmq
        ;;
    logs)
        echo "Fetching logs for RabbitMQ container..."
        docker logs -f rabbitmq
        ;;
    *)
        echo "Usage: $0 {build|start|stop|logs}"
        exit 1
esac
