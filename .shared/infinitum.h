/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_H
#define INFINITUM_H

#ifndef NOVUS_BITSIZE
#define NOVUS_BITSIZE 8
#endif

#include "export.h"
#include "novus.h"
#include "novus8.h"
#include "novus16.h"
#include "novus32.h"
#include "novus64.h"
#include "tempus.h"
#include "canor.h"
#include "aether.h"
#include "envoy.h"
#include "gui.h"

#ifdef __cplusplus
extern "C" {
#endif

#define INFINITUM_SHARED_ABI_VERSION 1u

INF_API uint32_t infinitum_shared_abi_version(void);
INF_API const char *infinitum_shared_name(void);
INF_API const char *infinitum_shared_platform_tag(void);

#ifdef __cplusplus
}
#endif

#endif
