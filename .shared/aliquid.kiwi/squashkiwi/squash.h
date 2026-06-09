/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_SQUASH_H
#define ALIQUIDKIWI_SQUASH_H

struct squash_t {
	struct squash_t *left;
	struct squash_t *right;
	
	size_t provider;
	size_t identifier;
	size_t header;
	size_t context;
	size_t content;
	size_t source;
	
	size_t number;
	struct tempus_t bases [];
};

#endif
