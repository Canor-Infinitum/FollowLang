/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_FLOW_H
#define FOLLOWLANG_FLOW_H

#include <infinitum/tempus.h>

struct flow_t {
	size_t items;
	struct tempus_base_t bases[];
};

struct flows_t {
	size_t length;
	struct flow_t flows [];
};

#endif
