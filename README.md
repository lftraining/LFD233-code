# LinuxFoundation Dapr WASM Course Lab Repository

This repository collects the hands on lab's for the trainings course [LFD233 (ADD LINK LATER)](). **To prevent potential issues with WASM thoroughout completing the labs, it’s advisable to switch to a Linux environment.**

**Labs**:
* **2.2**: [A Web Service in Rust Using hyper](./2.2/)
* **2.3**: [Deploying a Rust Web Service with WasmEdge](./2.3/)
* **3.3**: [Page Counter](./3.3/)
* **4.2**: [Using the Dapr Rust SDK](./4.2/)
* **4.3**: [Pub Sub with Dapr](./4.3/)
* **5.3**: [Monitoring Applications on Kubernetes](./5.3/)
* **6.2**: [Subscribing to Configuration Changes](./6.2/)
* **6.3**: [Retrieving Secrets with Dapr Using the Kubernetes Secrets Provider](./6.3/)
* **7.2**: [Time-Triggered SQL Statements](./7.2/)
* **8.2**: [WASM middleware](./8.2/)
* **8.3**: [WASM Output Bindings](./8.3/)

- [LinuxFoundation Dapr WASM Course Lab Repository](#linuxfoundation-dapr-wasm-course-lab-repository)
  - [Installs](#installs)
  - [Contact](#contact)

## Installs

To complete the labs in this course you need several applications installed. **To prevent potential issues with WASM thoroughout completing the labs, it’s advisable to switch to a Linux environment**. It should be safe to install the latest version; if problems occur, you may want to fall back to the specified "version used".

* **Core**:
  * Rust:
    * Link: https://www.rust-lang.org/tools/install 
    * Version used: `cargo 1.76.0`, `rustc 1.76.0`
  * Dapr: 
    * Link: https://docs.dapr.io/getting-started/install-dapr-cli/
    * Version used: `CLI version: 1.12.0`, `Runtime version: 1.12.5`
  * Docker:
    * Link: https://docs.docker.com/engine/install/
    * Version used: `Docker version 24.0.7`
  * WasmEdge:
    * Link: https://wasmedge.org/docs/start/install/
    * Version used: `wasmedge version 0.13.5`
* **Kubernetes related**:
  * Minikube: 
    * Link: https://k8s-docs.netlify.app/en/docs/tasks/tools/install-minikube/ 
    * Version used: `minikube version: v1.32.0`
  * Kubectl: 
    * Link: https://kubernetes.io/docs/tasks/tools/
    * Version used: `Client Version: v1.28.4`
  * Helm:
    * Link: https://helm.sh/docs/intro/install/
    * Version used: `version.BuildInfo{Version:"v3.13.3", GitCommit:"c8b948945e52abba22ff885446a1486cb5fd3474", GitTreeState:"clean", GoVersion:"go1.20.11"}`

## Contact

If there are problems with the resources contact XXX.
