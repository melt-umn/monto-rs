#ifndef MONTO_RS_CLIENT_H
#define MONTO_RS_CLIENT_H

#include <stdint.h> 

// The configuration to create a new Client.
typedef struct {
	// The hostname of the broker. Defaults to "localhost" if null.
	const char* hostname;

	// The port of the broker. Defaults to 28888 if zero.
	uint16_t port;

	// The identifier of the client. An identifier is a string that matches the
	// regex:
	//
	//     [a-zA-Z_][a-zA-Z_0-9]*(\\.[a-zA-Z_][a-zA-Z_0-9]*)+
	//
	// See the spec for more details.
	const char* identifier;

	// The name of the client. May be null.
	const char* name;

	// The vendor of the client. May be null.
	const char* vendor;

	// The major version of the client.
	unsigned int major;

	// The minor version of the client.
	unsigned int minor;

	// The patch version of the client.
	unsigned int patch;
} ClientConfig;

// Don't rely on *anything* about the Client type, other than that it will be a
// data pointer (rather than a code pointer). And if you're on an architecture
// that differentiates between the two, perhaps reconsider using this library.
//
// Additionally, Client is unsynchronized, so don't try to access it from
// multiple threads. This may change in the future, but for now will cause race
// conditions.
typedef struct Client Client;

// Instantiates a new Client, which must be freed with monto_rs_Client_free.
// This function also does not free the members of config.
//
// Returns null if an error occurs. In the future, this will be changed to have
// a proper error-handling mechanism, but no ADTs in C. :(
Client* monto_rs_Client_new(ClientConfig config);

// Frees the given Client. Also closes the connection to the Broker.
void monto_rs_Client_free(Client* client);

#endif
