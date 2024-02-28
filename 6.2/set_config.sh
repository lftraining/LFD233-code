#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 website_url interval_seconds"
    exit 1
fi

website_url=$1
interval_seconds=$2

website_url_key="website_url"
interval_seconds_key="interval_seconds"

# Redis command to set the key-value pairs
redis-cli SET $website_url_key $website_url
redis-cli SET $interval_seconds_key $interval_seconds

echo "Configuration set successfully:"
echo "Website URL: $website_url"
echo "Interval Seconds: $interval_seconds"
