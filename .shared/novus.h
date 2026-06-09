/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_NOVUS_H
#define INFINITUM_NOVUS_H

#include <stddef.h>
#include <stdint.h>

#ifndef NOVUS_BITSIZE
#define NOVUS_BITSIZE 8
#endif

#include "novus8.h"
#include "novus16.h"
#include "novus32.h"
#include "novus64.h"
#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

struct novus_t {
#if NOVUS_BITSIZE == 8
	struct novus_int8_t data;
	struct novus_token8_t settings;
	struct novus_value8_t update;
	struct novus_jitter8_t jitter;
#elif NOVUS_BITSIZE == 16
	struct novus_int16_t data;
	struct novus_token16_t settings;
	struct novus_value16_t update;
	struct novus_jitter16_t jitter;
#elif NOVUS_BITSIZE == 32
	struct novus_int32_t data;
	struct novus_token32_t settings;
	struct novus_value32_t update;
	struct novus_jitter32_t jitter;
#elif NOVUS_BITSIZE == 64
	struct novus_int64_t data;
	struct novus_token64_t settings;
	struct novus_value64_t update;
	struct novus_jitter64_t jitter;
#else
	size_t data;
	size_t settings;
	size_t update;
	size_t jitter;
#endif
};

INF_API uint32_t infinitum_novus_abi_version(void);
INF_API void novus8_zero(struct novus8_t *value);
INF_API void novus16_zero(struct novus16_t *value);
INF_API void novus32_zero(struct novus32_t *value);
INF_API void novus64_zero(struct novus64_t *value);

#ifdef __cplusplus
}
#endif

#endif
