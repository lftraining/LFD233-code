#!/bin/bash

SECRET_NAME="encryption-key-secret"
KEY_NAME="encryptionKey"

# Generate a random 256-bit (32-byte) encryption key and encode it in base64
ENCRYPTION_KEY=$(openssl rand -base64 32)

# Update the Kubernetes secret
kubectl create secret generic $SECRET_NAME --from-literal=$KEY_NAME="$ENCRYPTION_KEY" --dry-run=client -o yaml | kubectl apply -f - 

echo "Secret '$SECRET_NAME' updated successfully with the new encryption key."
