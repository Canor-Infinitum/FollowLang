/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_NOVUS64_H
#define INFINITUM_NOVUS64_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct novus_int64_t {
	uint64_t lower;
	uint64_t upper;
};

struct novus_token64_t {
	struct novus_int64_t minimum;
	struct novus_int64_t nominal;
	struct novus_int64_t maximum;
};

struct novus_value64_t {
	struct novus_token64_t pre;
	struct novus_token64_t peri;
	struct novus_token64_t post;
};

struct novus_jitter64_t {
	struct novus_value64_t low;
	struct novus_value64_t value;
	struct novus_value64_t high;
};

struct novus64_t {
	struct novus_int64_t data;
	struct novus_token64_t settings;
	struct novus_value64_t update;
	struct novus_jitter64_t jitter;
};

#ifdef __cplusplus
}
#endif

#endif
