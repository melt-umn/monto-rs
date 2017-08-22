#ifndef MONTO_RS_SERVICE_H
#define MONTO_RS_SERVICE_H

#include <stdint.h> 

// The configuration to create a new Service.
typedef struct {
	// The address to serve on. Defaults to all addresses ("0.0.0.0") if null.
	const char* addr;

	// The extensions to declare as being in use. If this is null, it is
	// treated as a zero-length list.
	const char** extensions;

	// The port of the broker. Defaults to 28888 if zero.
	uint16_t port;

	// The identifier of the service. An identifier is a string that matches the
	// regex:
	//
	//     [a-zA-Z_][a-zA-Z_0-9]*(\\.[a-zA-Z_][a-zA-Z_0-9]*)+
	//
	// See the spec for more details.
	const char* identifier;

	// The name of the service. May be null.
	const char* name;

	// The vendor of the service. May be null.
	const char* vendor;

	// The major version of the service.
	unsigned int major;

	// The minor version of the service.
	unsigned int minor;

	// The patch version of the service.
	unsigned int patch;
} ServiceConfig;

typedef struct Service Service;

// Instantiates a new Service, which must be freed with monto_rs_Service_free.
// This function also does not free the members of config.
//
// Returns null if an error occurs.
Service* monto_rs_Service_new(ServiceConfig config);

// Serves "forever". It may still exit if an I/O error occurs.
void monto_rs_Service_serve_forever(Service *service);

// Frees the given Service.
void monto_rs_Service_free(Service* service);

#endif
