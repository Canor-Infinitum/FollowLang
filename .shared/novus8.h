/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_NOVUS8_H
#define INFINITUM_NOVUS8_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct novus_int8_t {
	uint8_t lower;
	uint8_t upper;
};

struct novus_token8_t {
	struct novus_int8_t minimum;
	struct novus_int8_t nominal;
	struct novus_int8_t maximum;
};

struct novus_value8_t {
	struct novus_token8_t pre;
	struct novus_token8_t peri;
	struct novus_token8_t post;
};

struct novus_jitter8_t {
	struct novus_value8_t low;
	struct novus_value8_t value;
	struct novus_value8_t high;
};

struct novus8_t {
	struct novus_int8_t data;
	struct novus_token8_t settings;
	struct novus_value8_t update;
	struct novus_jitter8_t jitter;
};

#ifdef __cplusplus
}
#endif

#endif