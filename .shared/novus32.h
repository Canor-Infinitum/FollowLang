/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_NOVUS32_H
#define INFINITUM_NOVUS32_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct novus_int32_t {
	uint32_t lower;
	uint32_t upper;
};

struct novus_token32_t {
	struct novus_int32_t minimum;
	struct novus_int32_t nominal;
	struct novus_int32_t maximum;
};

struct novus_value32_t {
	struct novus_token32_t pre;
	struct novus_token32_t peri;
	struct novus_token32_t post;
};

struct novus_jitter32_t {
	struct novus_value32_t low;
	struct novus_value32_t value;
	struct novus_value32_t high;
};

struct novus32_t {
	struct novus_int32_t data;
	struct novus_token32_t settings;
	struct novus_value32_t update;
	struct novus_jitter32_t jitter;
};

#ifdef __cplusplus
}
#endif

#endif
