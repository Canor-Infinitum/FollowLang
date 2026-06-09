/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_AETHER_H
#define INFINITUM_AETHER_H

#include "novus8.h"
#include "canor.h"
#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

struct aether_color_t {
	struct novus8_t red;
	struct novus8_t green;
	struct novus8_t blue;
	struct novus8_t alpha;
};

struct aether_pixel8_t {
	struct aether_color_t color;
	struct canor8_t center;
};

struct aether_pixel16_t {
	struct aether_color_t color;
	struct canor16_t center;
};

struct aether_pixel32_t {
	struct aether_color_t color;
	struct canor32_t center;
};

struct aether_pixel64_t {
	struct aether_color_t color;
	struct canor64_t center;
};

struct aether_t {
	struct aether_color_t color;
	struct canor8_t coarse;
	struct canor16_t medium;
	struct canor32_t fine;
	struct canor64_t details;
};

INF_API void aether_color_init_rgba8(struct aether_color_t *color, uint8_t red, uint8_t green, uint8_t blue, uint8_t alpha);
INF_API void aether_init(struct aether_t *value);
INF_API void aether_free(struct aether_t *value);

#ifdef __cplusplus
}
#endif

#endif
