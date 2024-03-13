package core

const (
	RECORD_TYPE_A                = 1
	RECORD_TYPE_NS               = 2
	CLASS_TYPE_IN                = 1
	RECURSION_DESIRED            = 1 << 8
	RECURSION_NOT_DESIRED        = 0
	DEFAULT_NUMBER_OF_QUESTIONS  = 1
	MAX_DNS_PACKET_SIZE_IN_BYTES = 1024
	ROOT_SERVER                  = "198.41.0.4"
	DEBUG                        = true
)
