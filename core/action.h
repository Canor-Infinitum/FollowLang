/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_ACTION_H
#define FOLLOWLANG_ACTION_H

struct action_t {
	size_t equivalue;
	size_t modulus;
	
	size_t addcoeff;
	size_t *additive;
	size_t mulcoeff;
	size_t *multiplier;
};

struct actions_t {
	size_t length;
	struct action_t actions [];
};

#endif
