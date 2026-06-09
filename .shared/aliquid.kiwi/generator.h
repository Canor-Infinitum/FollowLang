/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_GENERATOR_H
#define ALIQUIDKIWI_GENERATOR_H

#include <strings.h>

#include <squashkiwi/squashkiwi.h>

struct aliquid_generator_t {
	struct aliquid_tokenizer_t tokens;
	size_t squashkiwis;
	struct squashkiwi_t squashkiwi [];
};

#endif
