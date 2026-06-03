/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_ELITE_H
#define FOLLOWLANG_ELITE_H

#include <infinitum/aether.h>
#include <infinitum/envoy.h>

#include <united.h>

struct token_t {
	const char *token;
	size_t length;
};

struct tokens_t {
	size_t count;
	struct token_t tokens[];
};

struct elite8_t {
	struct aether_t source_real;
	struct envoy_t source_base;
	struct aether_t *target_real;
	struct envoy_t *target_base;
	struct operator8_t op;
	struct tokens_t tokens;
};

struct elite16_t {
	struct aether_t source_real;
	struct envoy_t source_base;
	struct aether_t *target_real;
	struct envoy_t *target_base;
	struct operator16_t op;
	struct tokens_t tokens;
};

struct elite32_t {
	struct aether_t source_real;
	struct envoy_t source_base;
	struct aether_t *target_real;
	struct envoy_t *target_base;
	struct operator32_t op;
	struct tokens_t tokens;
};

struct elite64_t {
	struct aether_t source_real;
	struct envoy_t source_base;
	struct aether_t *target_real;
	struct envoy_t *target_base;
	struct operator64_t op;
	struct tokens_t tokens;
};

struct elite_t {
	struct aether_t source_real;
	struct envoy_t source_base;
	struct aether_t *target_real;
	struct envoy_t *target_base;
	struct operator_t op;
	struct tokens_t tokens;
};

#endif
