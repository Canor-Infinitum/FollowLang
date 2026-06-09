#include <string.h>
#include "canor.h"

#define DEFINE_CANOR_INIT(BITS) \
void canor##BITS##_init(struct canor##BITS##_t *value) { \
    if (value == NULL) { \
        return; \
    } \
    memset(value, 0, sizeof(*value)); \
    tempus_init(&value->tempus); \
}

#define DEFINE_CANOR_COPY(BITS) \
int canor##BITS##_copy(struct canor##BITS##_t *dst, const struct canor##BITS##_t *src) { \
    if (dst == NULL || src == NULL) { \
        return -1; \
    } \
    memcpy(&dst->novus, &src->novus, sizeof(dst->novus)); \
    return tempus_copy(&dst->tempus, &src->tempus); \
}

#define DEFINE_CANOR_FREE(BITS) \
void canor##BITS##_free(struct canor##BITS##_t *value) { \
    if (value == NULL) { \
        return; \
    } \
    tempus_free(&value->tempus); \
}

DEFINE_CANOR_INIT(8)
DEFINE_CANOR_INIT(16)
DEFINE_CANOR_INIT(32)
DEFINE_CANOR_INIT(64)

DEFINE_CANOR_COPY(8)
DEFINE_CANOR_COPY(16)
DEFINE_CANOR_COPY(32)
DEFINE_CANOR_COPY(64)

DEFINE_CANOR_FREE(8)
DEFINE_CANOR_FREE(16)
DEFINE_CANOR_FREE(32)
DEFINE_CANOR_FREE(64)
``