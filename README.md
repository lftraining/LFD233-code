# LinuxFoundation Dapr WASM Course Lab Repository

This repository collects the hands on lab's for the trainings course [LFD233 (ADD LINK LATER)]().

* **2.2**: [A Web Service in Rust Using hyper](./2.2/)
* **2.3**: [Deploying a Rust Web Service with WasmEdge](./2.3/)
* **3.3**: [Page Counter](./3.3/)
* **4.2**: [Using the Dapr Rust SDK](./4.2/)
* **4.3**: [Pub Sub with Dapr](./4.3/)
* **5.3**: [Monitoring Applications on Kubernetes](./5.3/)
* **6.2**: [Subscribing to Configuration Changes](./6.2/)
* **6.3**: [Retrieving Secrets with Dapr Using the Kubernetes Secrets Provider](./6.3/)
* **7.2**: [Time-Triggered SQL Statements](./7.2/)
* **8.2**: [(**TBD**) Wasm middleware](./8.2/)
* **8.3**: [(**TBD**) Wasm Output Bindings](./8.3/)

- [LinuxFoundation Dapr WASM Course Lab Repository](#linuxfoundation-dapr-wasm-course-lab-repository)
  - [Dapr with WebAssembly - Detailed Course Outline](#dapr-with-webassembly---detailed-course-outline)
    - [Ch 1. Dapr Overview](#ch-1-dapr-overview)
    - [Ch 2. Building a Wasm Web Service](#ch-2-building-a-wasm-web-service)
    - [Ch 3. Dapr with WasmEdge](#ch-3-dapr-with-wasmedge)
    - [Ch 4. State Management with Databases](#ch-4-state-management-with-databases)
    - [Ch 5. Dapr Applications on Kubernetes](#ch-5-dapr-applications-on-kubernetes)
    - [Ch 6. Configuration and Secret Management](#ch-6-configuration-and-secret-management)
    - [Ch 7. Input and Output Binding](#ch-7-input-and-output-binding)
    - [Ch 8. Running Wasm in Dapr](#ch-8-running-wasm-in-dapr)
  - [Contact](#contact)

## Dapr with WebAssembly - Detailed Course Outline

### Ch 1. Dapr Overview
In this chapter we will get a comprehensive introduction to Dapr, an open-source project under the Cloud Native Computing Foundation (CNCF), focusing on its role in microservices architecture. Dapr offers a versatile framework that allows seamless integration of services without directly altering application code. It emphasizes decoupling, enabling development teams to focus on business logic while infrastructure teams manage the underlying topology. Further, we will delve into Dapr's building blocks, its implementation in microservices, and its integration with WebAssembly (WASM), highlighting how Dapr and WASM together streamline development processes in cloud-native environments.

* Motivation
* Introduction to Dapr
* Introducing Dapr's Building Blocks
* Dapr with WebAssembly

At the end of this chapter, learners have a good understanding of the advantages of WebAssembly (polyglot, platform-independent, small, secure) and how Dapr provides a variety of building blocks for the young ecosystem.

### Ch 2. Building a Wasm Web Service
The chapter is structured to get you familiar with WebAssembly (WASM) runtimes and the development of a web service in Rust. We start by discussing the concept of WASM runtimes. Then we shift to practical applications, starting with the development of a web service using Rust. This is followed by an in-depth look at deploying this service using WasmEdge, emphasizing the compilation of Rust applications to WebAssembly.

* WASM Runtimes
* [Lab: A Web Service in Rust using Hyper](./2.2/)
* [Lab: Deploying a Rust Web Service with WasmEdge](./2.3/)

At the end of this chapter, learners know what runtimes are available, why WasmEdge was chosen to demonstrate the capabilities of Daprs in this course, how to build a simple Rust web service, and how to run it using the WasmEdge runtime.

### Ch 3. Dapr with WasmEdge
In this chapter, we will further discuss Dapr. First, we'll explore in greater detail how Dapr can be integrated with a web service. Following that, we'll conduct a practical lab to implement this integration. In the second lab, we will create a basic web service to develop a page counter, utilizing Dapr’s state management capabilities.

* Connecting Dapr to a Web Service
* Lab: Initialize Dapr
* [Lab: Page Counter Application](./3.3/)

At the end of this chapter, learners can set up Dapr locally (on Docker) and know how to use it in a simple application to store information in a key-value store.

### Ch 4. State Management with Databases
This chapter provides an introduction to the fundamentals of storing data in databases with the Dapr StateStore building block and facilitating loose communication between services using the Dapr publish/subscribe (Pub/Sub) building block. First, we will get an overview of the components. After that, we will develop in a lab using the State Store Dapr component. The first lab will use the Dapr Rust SDK to store and retrieve data. In the second lab, we will walk through the use of the Pub/Sub component.

* State Store and Pub/Sub in Dapr
* [Lab: Using the Dapr Rust SDK to store data](./4.2/)
* [Lab: Dapr Publish Subscribe](./4.3/)

At the end of this chapter, learners can access databases with Dapr, understand how the Dapr SDK can be used to simplify communication with the Dapr sidecar, and know how to use the pub-sub component to establish asynchronous communication between applications.

### Ch 5. Dapr Applications on Kubernetes
In this chapter we will take a look at how to use Dapr within a Kubernetes environment. We start by exploring Dapr’s components in a Kubernetes cluster, highlighting the roles of the Dapr Placement Service, Sidecar Injector, Operator, and Sentry. Through a practical lab session, we will gain a thorough understanding of deploying and managing Dapr-enabled applications in Kubernetes.

* Dapr Architecture on Kubernetes
* [Lab: Install Dapr on Kubernetes](./5.3/)

At the end of this chapter, learners can prepare a Kubernetes cluster to run Dapr applications.

### Ch 6. Configuration and Secret Management
In this chapter we will talk about managing configurations and secrets with Dapr. We will look at the practical aspects of integrating external configurations into the application, which we will develop in the lab in this chapter, to enable dynamic responses to configuration changes at runtime. Furthermore, we will navigate through Dapr's secret management, understanding how to securely store and retrieve sensitive information. Through engaging theoretical discussions and hands-on lab exercises, we uncover the best practices for managing configurations and secrets, ensuring applications are robust, flexible, and secure.

* Configuration Management in Dapr
* [Lab: Subscribing to Configuration Changes](6.2/)
* [Lab: Retrieving Secrets with Dapr Using the Kubernetes Secrets Provider](./6.3/)

At the end of this chapter, learners can integrate external configuration into an application, know how to subscribe to configuration to change the behavior of the application at runtime. How to manage secrets by using the Dapr secrets component.

### Ch 7. Input and Output Binding
In this chapter, you will delve into the practical and theoretical aspects of Dapr's bindings, focusing on input and output bindings as mechanisms for connecting applications with external services and systems. Through a combination of theoretical explanations and hands-on lab exercises, you will learn how to use these bindings effectively to automate and streamline time-based database operations. The chapter starts with an in-depth look at the variety of bindings supported by Dapr, setting the stage for the subsequent lab where you'll put theory into practice, using Cron Input and SQL Output bindings to trigger and manage time-based jobs in a PostgreSQL database.

* Overview of Dapr Bindings
* [Lab: Time-Triggered SQL Statements](7.2/)

At the end of this chapter, learners will know what bindings are available and to use the Cron Input and SQL Output bindings to trigger time-based database jobs.

### Ch 8. Running Wasm in Dapr
In this final chapter, we will look at how we can use WebAssembly to extend Dapr. For this, we don't need an external WASM runtime, since Dapr comes with an integrated runtime for running custom middleware components and providing custom logic as output bindings. The chapter starts with a closer look at Wazero, the built-in WASM runtime. We then create a WASM output binding and finally create WASM-based middleware to implement custom routing logic.

* Dapr onboard Runtime (Wazero)
* [Lab: Wasm middleware](./8.2/)
* [Lab: Wasm Output Bindings](8.3/)

At the end of this chapter, learners know what features Dapr offers with its internal Wazero runtime. It describes how to use filters as middleware and how to provide Wasm modules to any application.

## Contact

If there are problems with the resources contact XXX.
