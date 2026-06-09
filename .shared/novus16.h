/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_NOVUS16_H
#define INFINITUM_NOVUS16_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct novus_int16_t {
	uint16_t lower;
	uint16_t upper;
};

struct novus_token16_t {
	struct novus_int16_t minimum;
	struct novus_int16_t nominal;
	struct novus_int16_t maximum;
};

struct novus_value16_t {
	struct novus_token16_t pre;
	struct novus_token16_t peri;
	struct novus_token16_t post;
};

struct novus_jitter16_t {
	struct novus_value16_t low;
	struct novus_value16_t value;
	struct novus_value16_t high;
};

struct novus16_t {
	struct novus_int16_t data;
	struct novus_token16_t settings;
	struct novus_value16_t update;
	struct novus_jitter16_t jitter;
};

#ifdef __cplusplus
}
#endif

#endif
