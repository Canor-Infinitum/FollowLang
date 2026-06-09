#include <stdlib.h>
#include <string.h>
#include "envoy.h"

void envoy_strategy_init(struct envoy_strategy_t *value) {
    if (value == NULL) {
        return;
    }
    memset(value, 0, sizeof(*value));
}

int envoy_strategy_reserve(struct envoy_strategy_t *value, uint64_t blocks) {
    struct tempus_t *chain;
    uint64_t i;
    if (value == NULL) {
        return -1;
    }
    if (blocks == 0u) {
        if (value->chain != NULL) {
            for (i = 0u; i < value->blocks; ++i) {
                tempus_free(&value->chain[i]);
            }
            free(value->chain);
        }
        value->chain = NULL;
        value->blocks = 0u;
        return 0;
    }
    if (blocks > (SIZE_MAX / sizeof(struct tempus_t))) {
        return -2;
    }
    chain = (struct tempus_t *)calloc((size_t)blocks, sizeof(struct tempus_t));
    if (chain == NULL) {
        return -3;
    }
    if (value->chain != NULL) {
        for (i = 0u; i < value->blocks; ++i) {
            tempus_free(&value->chain[i]);
        }
        free(value->chain);
    }
    for (i = 0u; i < blocks; ++i) {
        tempus_init(&chain[i]);
    }
    value->chain = chain;
    value->blocks = blocks;
    return 0;
}

int envoy_strategy_copy(struct envoy_strategy_t *dst, const struct envoy_strategy_t *src) {
    uint64_t i;
    if (dst == NULL || src == NULL) {
        return -1;
    }
    envoy_strategy_free(dst);
    dst->left = src->left;
    dst->right = src->right;
    if (src->blocks == 0u || src->chain == NULL) {
        return 0;
    }
    if (envoy_strategy_reserve(dst, src->blocks) != 0) {
        return -2;
    }
    for (i = 0u; i < src->blocks; ++i) {
        if (tempus_copy(&dst->chain[i], &src->chain[i]) != 0) {
            envoy_strategy_free(dst);
            return -3;
        }
    }
    return 0;
}

void envoy_strategy_free(struct envoy_strategy_t *value) {
    uint64_t i;
    if (value == NULL) {
        return;
    }
    if (value->chain != NULL) {
        for (i = 0u; i < value->blocks; ++i) {
            tempus_free(&value->chain[i]);
        }
        free(value->chain);
    }
    value->chain = NULL;
    value->blocks = 0u;
    value->left = NULL;
    value->right = NULL;
}

void envoy_voxel_init(struct envoy_voxel_t *value) {
    if (value == NULL) {
        return;
    }
    memset(value, 0, sizeof(*value));
    envoy_strategy_init(&value->structure);
}

int envoy_voxel_reserve_pixels(struct envoy_voxel_t *value, uint64_t count) {
    struct aether_t *pixels;
    uint64_t i;
    if (value == NULL) {
        return -1;
    }
    if (count == 0u) {
        if (value->pixels != NULL) {
            for (i = 0u; i < value->count; ++i) {
                aether_free(&value->pixels[i]);
            }
            free(value->pixels);
        }
        value->pixels = NULL;
        value->count = 0u;
        return 0;
    }
    if (count > (SIZE_MAX / sizeof(struct aether_t))) {
        return -2;
    }
    pixels = (struct aether_t *)calloc((size_t)count, sizeof(struct aether_t));
    if (pixels == NULL) {
        return -3;
    }
    if (value->pixels != NULL) {
        for (i = 0u; i < value->count; ++i) {
            aether_free(&value->pixels[i]);
        }
        free(value->pixels);
    }
    for (i = 0u; i < count; ++i) {
        aether_init(&pixels[i]);
    }
    value->pixels = pixels;
    value->count = count;
    return 0;
}

int envoy_voxel_copy(struct envoy_voxel_t *dst, const struct envoy_voxel_t *src) {
    uint64_t i;
    if (dst == NULL || src == NULL) {
        return -1;
    }
    envoy_voxel_free(dst);
    dst->left = src->left;
    dst->right = src->right;
    dst->x = src->x;
    dst->y = src->y;
    dst->z = src->z;
    dst->width = src->width;
    dst->height = src->height;
    dst->depth = src->depth;
    if (envoy_strategy_copy(&dst->structure, &src->structure) != 0) {
        return -2;
    }
    if (src->count == 0u || src->pixels == NULL) {
        return 0;
    }
    if (envoy_voxel_reserve_pixels(dst, src->count) != 0) {
        envoy_voxel_free(dst);
        return -3;
    }
    for (i = 0u; i < src->count; ++i) {
        memcpy(&dst->pixels[i].color, &src->pixels[i].color, sizeof(dst->pixels[i].color));
        if (canor8_copy(&dst->pixels[i].coarse, &src->pixels[i].coarse) != 0 ||
            canor16_copy(&dst->pixels[i].medium, &src->pixels[i].medium) != 0 ||
            canor32_copy(&dst->pixels[i].fine, &src->pixels[i].fine) != 0 ||
            canor64_copy(&dst->pixels[i].details, &src->pixels[i].details) != 0) {
            envoy_voxel_free(dst);
            return -4;
        }
    }
    return 0;
}

void envoy_voxel_free(struct envoy_voxel_t *value) {
    uint64_t i;
    if (value == NULL) {
        return;
    }
    envoy_strategy_free(&value->structure);
    if (value->pixels != NULL) {
        for (i = 0u; i < value->count; ++i) {
            aether_free(&value->pixels[i]);
        }
        free(value->pixels);
    }
    value->pixels = NULL;
    value->count = 0u;
    value->left = NULL;
    value->right = NULL;
}