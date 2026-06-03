/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_ETHER_H
#define FOLLOWLANG_ETHER_H

#include <infinitum/novus.h>
#include <infinitum/novus8.h>
#include <infinitum/novus16.h>
#include <infinitum/novus32.h>
#include <infinitum/novus64.h>

#include <follower.h>

struct ether8_t {
	struct novus8_t update;
	struct novus8_t epsilon;
	struct novus8_t error;
	
	size_t followers;
	struct follower_t follower [];
};

struct ether16_t {
	struct novus16_t update;
	struct novus16_t epsilon;
	struct novus16_t error;
	
	size_t followers;
	struct follower_t follower [];
};

struct ether32_t {
	struct novus32_t update;
	struct novus32_t epsilon;
	struct novus32_t error;
	
	size_t followers;
	struct follower_t follower [];
};

struct ether64_t {
	struct novus64_t update;
	struct novus64_t epsilon;
	struct novus64_t error;
	
	size_t followers;
	struct follower_t follower [];
};

struct ether_t {
	struct novus_t update;
	struct novus_t epsilon;
	struct novus_t error;
	
	size_t followers;
	struct follower_t follower [];
};

#endif
