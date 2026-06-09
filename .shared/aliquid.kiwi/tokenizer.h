/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_TOKENIZER_H
#define ALIQUIDKIWI_TOKENIZER_H

struct aliquid_token_t {
	size_t length;
	const char *token;
};

struct aliquid_tokens_t {
	struct aliquid_token_t pre;
	struct aliquid_token_t peri;
	struct aliquid_token_t post;
};

struct aliquid_tokenizer_t {
	size_t count;
	struct aliquid_tokens_t list [];
};

#endif
