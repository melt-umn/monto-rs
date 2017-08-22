#include <monto_rs/service.h>
#include <stdio.h>

int main(void) {
	// Initialize the service.
	Service* service = monto_rs_Service_new((ServiceConfig) {
		.addr = "0.0.0.0",
		.extensions = NULL,
		.port = 28888,

		.identifier = "edu.umn.cs.melt.monto_rs_c_example_service",
		.name = "Example Service",
		.vendor = "MELT",
		.major = 0,
		.minor = 1,
		.patch = 0,
	});
	// TODO Add ServiceProviders!

	monto_rs_Service_serve_forever(service);
}
