#include <stdlib.h>
#include <string.h>
#include "tempus.h"

void tempus_basum_init(struct tempus_basum_t *basum, bool sign, uint8_t base, uint64_t ring) {
    if (basum == NULL) {
        return;
    }
    basum->sign = sign;
    basum->base = base;
    basum->reserved16 = 0u;
    basum->ring = ring;
}

void tempus_base_init(struct tempus_base_t *base) {
    if (base == NULL) {
        return;
    }
    memset(base, 0, sizeof(*base));
}

void tempus_init(struct tempus_t *value) {
    if (value == NULL) {
        return;
    }
    memset(value, 0, sizeof(*value));
}

int tempus_reserve(struct tempus_t *value, uint64_t items) {
    struct tempus_base_t *bases;
    if (value == NULL) {
        return -1;
    }
    if (items == 0u) {
        free(value->bases);
        value->bases = NULL;
        value->items = 0u;
        return 0;
    }
    if (items > (SIZE_MAX / sizeof(struct tempus_base_t))) {
        return -2;
    }
    bases = (struct tempus_base_t *)calloc((size_t)items, sizeof(struct tempus_base_t));
    if (bases == NULL) {
        return -3;
    }
    free(value->bases);
    value->bases = bases;
    value->items = items;
    return 0;
}

int tempus_copy(struct tempus_t *dst, const struct tempus_t *src) {
    if (dst == NULL || src == NULL) {
        return -1;
    }
    tempus_free(dst);
    dst->left = src->left;
    dst->right = src->right;
    if (src->items == 0u || src->bases == NULL) {
        dst->items = 0u;
        dst->bases = NULL;
        return 0;
    }
    if (tempus_reserve(dst, src->items) != 0) {
        return -2;
    }
    memcpy(dst->bases, src->bases, (size_t)src->items * sizeof(struct tempus_base_t));
    return 0;
}

void tempus_free(struct tempus_t *value) {
    if (value == NULL) {
        return;
    }
    free(value->bases);
    value->bases = NULL;
    value->items = 0u;
    value->left = NULL;
    value->right = NULL;
}

char tempus_base_symbol(uint8_t base) {
    switch (base) {
        case TEMPUS_BASE_ONE: return '1';
        case TEMPUS_BASE_I:   return 'i';
        case TEMPUS_BASE_J:   return 'j';
        case TEMPUS_BASE_K:   return 'k';
        default:              return '?';
    }
}