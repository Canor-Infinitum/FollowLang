/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_FOLLOWER_H
#define FOLLOWLANG_FOLLOWER_H

#include <elite.h>

struct follower_t {
	struct follower_t *parent;
	struct follower_t *left;
	struct follower_t *right;
	
	size_t update_count;
	size_t epsilon_count;
	size_t error_count;
	
	size_t length;
	struct tokens_t data [];
};

#endif
