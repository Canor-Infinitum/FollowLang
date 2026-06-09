#include "infinitum.h"

uint32_t infinitum_shared_abi_version(void) {
    return INFINITUM_SHARED_ABI_VERSION;
}

const char *infinitum_shared_name(void) {
    return "Canor Aether Infinitum - Shared Library";
}

const char *infinitum_shared_platform_tag(void) {
#if defined(_WIN32)
    return ".shared.dll";
#elif defined(__APPLE__)
    return ".shared.dylib";
#else
    return ".shared.so";
#endif
}
