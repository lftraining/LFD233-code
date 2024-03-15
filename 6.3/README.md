# Dapr Secrets

This lab is build to illustrate how secrets can be used in Dapr. In this application we retrieve an encryption key which we use in an app to encrypt and decrypt a string. We store the secret as Kubernetes Secret but give access to the applicaiton with the Dapr secret building block. The diagram below shows the system setup.

![Architecture Overview](./6.3%20dapr%20wasm%20training%20dapr%20secrets%20crypt.drawio.png)

All applications that are used throughout the entire course are listed under [Installs](https://github.com/lftraining/LFD233-code/?tab=readme-ov-file#installs).

**How complete the lab**:
1. **Setup** [K8s cluster](https://minikube.sigs.k8s.io/docs/) and install Dapr.
2. **Deploy the K8s components**: `kubectl apply -f deployment`
3. **The secret is empty just yet**: `kubectl get secret -o yaml encryption-key-secret`
4. **Update the secret** (`create_encryption_key_secret.sh`):
   1. `ENCRYPTION_KEY=$(openssl rand -base64 32)`
   2. `kubectl create secret generic $SECRET_NAME --from-literal=$KEY_NAME="$ENCRYPTION_KEY" --dry-run=client -o yaml | kubectl apply -f -` 
5. **Port forward**: `kubectl port-forward svc/secrets-crypt 8080:8080`
6. **Test the App**: you can visit localhost:8080 you should see `Secrets Crypt App Runs!`. After that you can test the encryption & decryption functionallity.
   1. `curl -X POST http://127.0.0.1:8080/encrypt -d 'some test string' > tmp.txt`
   2. `curl -X POST http://127.0.0.1:8080/decrypt -d "$(cat tmp.txt)"`

Done!

## Further Commands - Update the container image:
* `docker build -f Containerfile -t leonardpahlke/secrets-crypt:latest .`
* `docker push leonardpahlke/secrets-crypt:latest`
* `kubectl rollout restart deployment/secrets-crypt`
