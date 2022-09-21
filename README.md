# microstack

A template microservice architecture running on k8s.

## Services

- Consul
	- Consul client - provides a local Consul instance for other services
	- Consul servers - 3 (by default) replicated Consul servers for providing service discovery.
- Postgres
- [`gateway-service`](./packages/gateway-service/) - an HTTP server interfacing between the Web and internal services.
- [`products-services`](./packages/products-service/) - a gRPC shim service providing information on products.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.
