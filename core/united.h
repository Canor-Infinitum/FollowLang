/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_UNITED_H
#define FOLLOWLANG_UNITED_H

#include <infinitum/aether.h>
#include <infinitum/envoy.h>

struct operator8_t {
	void (*proc)(struct aether_t, struct envoy_t, struct *maneuver8_t, size_t maneuver_len);
	
	size_t size;
	struct maneuver8_t maneuvers [];
};

struct operator16_t {
	void (*proc)(struct aether_t, struct envoy_t, struct *maneuver16_t, size_t maneuver_len);
	
	size_t size;
	struct maneuver16_t maneuvers [];
};

struct operator32_t {
	void (*proc)(struct aether_t, struct envoy_t, struct *maneuver32_t, size_t maneuver_len);
	
	size_t size;
	struct maneuver32_t maneuvers [];
};

struct operator64_t {
	void (*proc)(struct aether_t, struct envoy_t, struct *maneuver64_t, size_t maneuver_len);
	
	size_t size;
	struct maneuver64_t maneuvers [];
};

struct operator_t {
	void (*proc)(struct aether_t, struct envoy_t, struct *maneuver_t, size_t maneuver_len);
	
	size_t size;
	struct maneuver_t maneuvers [];
};

struct united8_t {
	struct aether_t real;
	struct envoy_t base;
	struct operator8_t op;
};

struct united16_t {
	struct aether_t real;
	struct envoy_t base;
	struct operator16_t op;
};

struct united32_t {
	struct aether_t real;
	struct envoy_t base;
	struct operator32_t op;
};

struct united64_t {
	struct aether_t real;
	struct envoy_t base;
	struct operator64_t op;
};

struct united_t {
	struct aether_t real;
	struct envoy_t base;
	struct operator_t op;
};

#endif
