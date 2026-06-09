/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_KIWI_H
#define ALIQUIDKIWI_KIWI_H

#include <infinitum/novus.h>
#include <infinitum/novus8.h>
#include <infinitum/novus16.h>
#include <infinitum/novus32.h>
#include <infinitum/novus64.h>

struct kiwi_t {
	struct novus_value8_t coarse;
	struct novus_value16_t medium;
	struct novus_value32_t fine;
	struct novus_value64_t details;
};

#endif
