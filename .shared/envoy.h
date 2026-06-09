
/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_ENVOY_H
#define INFINITUM_ENVOY_H

#include <stdint.h>
#include "aether.h"
#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

struct envoy_strategy_t {
	struct envoy_strategy_t *left;
	struct envoy_strategy_t *right;
	
	size_t blocks;
	struct tempus_t *chain;
};

struct envoy_voxel_t {
	struct envoy_voxel_t *left;
	struct envoy_voxel_t *right;
	
	struct envoy_strategy_t structure;
	
	size_t x;
	size_t y;
	size_t z;
	size_t width;
	size_t height;
	size_t depth;
	
	size_t count;
	struct aether_t *pixels;
};

INF_API void envoy_strategy_init(struct envoy_strategy_t *value);
INF_API int envoy_strategy_reserve(struct envoy_strategy_t *value, uint64_t blocks);
INF_API int envoy_strategy_copy(struct envoy_strategy_t *dst, const struct envoy_strategy_t *src);
INF_API void envoy_strategy_free(struct envoy_strategy_t *value);

INF_API void envoy_voxel_init(struct envoy_voxel_t *value);
INF_API int envoy_voxel_reserve_pixels(struct envoy_voxel_t *value, uint64_t count);
INF_API int envoy_voxel_copy(struct envoy_voxel_t *dst, const struct envoy_voxel_t *src);
INF_API void envoy_voxel_free(struct envoy_voxel_t *value);

#ifdef __cplusplus
}
#endif

#endif
