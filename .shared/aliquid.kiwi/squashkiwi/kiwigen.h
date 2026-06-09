/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_KIWIGEN_H
#define ALIQUIDKIWI_KIWIGEN_H

#include <kiwi.h>

struct kiwigen_t {
	struct kiwigen_t *left;
	struct kiwigen_t *right;
	
	size_t provider;
	size_t identifier;
	size_t header;
	size_t context;
	size_t content;
	size_t source;
	
	size_t number;
	struct kiwi_t kiwi [];
};

#endif
