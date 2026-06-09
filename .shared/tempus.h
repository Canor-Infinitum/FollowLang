
/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_TEMPUS_H
#define INFINITUM_TEMPUS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

enum tempus_bases {
    TEMPUS_BASE_ONE = 0,
    TEMPUS_BASE_I   = 1,
    TEMPUS_BASE_J   = 2,
    TEMPUS_BASE_K   = 3
};

struct tempus_basum_t {
	bool sign;
	enum tempus_bases base : 2;
	size_t ring;
};

struct tempus_base_t {
	struct tempus_base_t *left;
	struct tempus_base_t *right;
	
	struct tempus_basum_t diffl;
	struct tempus_basum_t diffr;
	struct tempus_basum_t value;
};

struct tempus_t {
	struct tempus_t *left;
	struct tempus_t *right;
	
	size_t items;
	struct tempus_base_t *bases;
};

INF_API void tempus_basum_init(struct tempus_basum_t *basum, bool sign, uint8_t base, uint64_t ring);
INF_API void tempus_base_init(struct tempus_base_t *base);
INF_API void tempus_init(struct tempus_t *value);
INF_API int tempus_reserve(struct tempus_t *value, uint64_t items);
INF_API int tempus_copy(struct tempus_t *dst, const struct tempus_t *src);
INF_API void tempus_free(struct tempus_t *value);
INF_API char tempus_base_symbol(uint8_t base);

#ifdef __cplusplus
}
#endif

#endif
