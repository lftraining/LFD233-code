
# Deploy an Application to Kubernetes

In this lab, we will install Dapr on a Kubernetes cluster. You will need access to a Kubernetes cluster to complete this lab. [Minikube](https://minikube.sigs.k8s.io/docs/) can be used to host a local Kubernetes cluster. 

All applications that are used throughout the entire course are listed under [Installs](https://github.com/lftraining/LFD233-code/?tab=readme-ov-file#installs).

**How complete the lab**:
1. Install Dapr in your Kubernetes cluster using Helm:
   1. `helm repo add dapr https://dapr.github.io/helm-charts/`
   2. `helm repo update`
   3. `helm install dapr dapr/dapr --namespace dapr-system --create-namespace`
2. Check the installation
   1. `kubectl get pods -n dapr-system`
   2. `kubectl get pods`
   3. `kubectl logs -l app=page-counter -c page-counter`
3. Port forward to your application and the sidecar:
   1. `kubectl -- port-forward $(kubectl get pods -l app=page-counter -o jsonpath="{.items[0].metadata.name}") 3500:3500`
   2. `kubectl port-forward $(kubectl get pods -l app=page-counter -o jsonpath="{.items[0].metadata.name}") 3500:3500`
4. Test the application by incrementing the page counter:
   1. `curl http://localhost:8081`
   2. `curl http://localhost:3500/v1.0/state/statestore/counter`

Done!