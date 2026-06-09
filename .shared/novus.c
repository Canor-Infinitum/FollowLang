#include <string.h>
#include "novus.h"

uint32_t infinitum_novus_abi_version(void) {
    return 1u;
}

void novus8_zero(struct novus8_t *value) {
    if (value != NULL) {
        memset(value, 0, sizeof(*value));
    }
}

void novus16_zero(struct novus16_t *value) {
    if (value != NULL) {
        memset(value, 0, sizeof(*value));
    }
}

void novus32_zero(struct novus32_t *value) {
    if (value != NULL) {
        memset(value, 0, sizeof(*value));
    }
}

void novus64_zero(struct novus64_t *value) {
    if (value != NULL) {
        memset(value, 0, sizeof(*value));
    }
}