/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_SQUASHKIWI_H
#define ALIQUIDKIWI_SQUASHKIWI_H

#include <kiwi.h>
#include <squash.h>

struct squashkiwi_t {
	struct squashkiwi_t *left;
	struct squashkiwi_t *right;
	
	size_t provider;
	size_t identifier;
	size_t header;
	size_t context;
	size_t content;
	size_t source;
	
	struct kiwigen_t kiwi;
	
	size_t squash_len;
	struct squash_t squashes [];
};

#endif
