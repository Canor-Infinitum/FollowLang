/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_REPORT_H
#define ALIQUIDKIWI_REPORT_H

struct aliquid_report_t {
	size_t length;
	struct aliquid_tokenizer_t tokens [];
};

#endif
