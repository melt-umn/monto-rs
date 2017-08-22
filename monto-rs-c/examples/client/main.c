#include <monto_rs/client.h>
#include <stdio.h>

int main(void) {
	// Initialize the client.
	puts("Connecting to broker...");
	Client* client = monto_rs_Client_new((ClientConfig) {
		.hostname = "localhost",
		.port = 28888,
		.identifier = "edu.umn.cs.melt.monto_rs_c_example_client",
		.name = "Example Client",
		.vendor = "MELT",
		.major = 0,
		.minor = 1,
		.patch = 0,
	});
	puts("Connected to broker.");

	// TODO Actually do something!

	// Free the client and close connections to the broker.
	puts("Disconnecting from broker and freeing client...");
	monto_rs_Client_free(client);
	puts("Disconnected from broker.");
	return 0;
}
