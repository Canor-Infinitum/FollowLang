/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_CANOR_H
#define INFINITUM_CANOR_H

#include "novus.h"
#include "tempus.h"
#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

struct canor8_t {
	struct novus8_t novus;
	struct tempus_t tempus;
};

struct canor16_t {
	struct novus16_t novus;
	struct tempus_t tempus;
};

struct canor32_t {
	struct novus32_t novus;
	struct tempus_t tempus;
};

struct canor64_t {
	struct novus64_t novus;
	struct tempus_t tempus;
};

struct canor_t {
#if NOVUS_BITSIZE == 8
	struct novus8_t novus;
#elif NOVUS_BITSIZE == 16
	struct novus16_t novus;
#elif NOVUS_BITSIZE == 32
	struct novus32_t novus;
#elif NOVUS_BITSIZE == 64
	struct novus64_t novus;
#else
	struct novus_t novus;
#endif

	struct tempus_t tempus;
};

INF_API void canor8_init(struct canor8_t *value);
INF_API void canor16_init(struct canor16_t *value);
INF_API void canor32_init(struct canor32_t *value);
INF_API void canor64_init(struct canor64_t *value);
INF_API int canor8_copy(struct canor8_t *dst, const struct canor8_t *src);
INF_API int canor16_copy(struct canor16_t *dst, const struct canor16_t *src);
INF_API int canor32_copy(struct canor32_t *dst, const struct canor32_t *src);
INF_API int canor64_copy(struct canor64_t *dst, const struct canor64_t *src);
INF_API void canor8_free(struct canor8_t *value);
INF_API void canor16_free(struct canor16_t *value);
INF_API void canor32_free(struct canor32_t *value);
INF_API void canor64_free(struct canor64_t *value);

#ifdef __cplusplus
}
#endif

#endif
