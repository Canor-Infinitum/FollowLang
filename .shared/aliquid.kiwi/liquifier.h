/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef ALIQUIDKIWI_LIQUIFIER_H
#define ALIQUIDKIWI_LIQUIFIER_H

#include <follow/flow.h>
#include <follow/action.h>
#include <follow/maneuver.h>
#include <follow/united.h>
#include <follow/elite.h>
#include <follow/follower.h>
#include <follow/ether.h>

#include <tokenizer.h>
#include <generator.h>

struct aliquifier_t {
	struct flows_t flows;
	struct actions_t actions;
	struct maneuver_t maneuver;
	struct united_t schema;
	struct elite_t programming;
	struct follower_t follower;
	
	struct ether8_t coarse;
	struct ether16_t medium;
	struct ether32_t fine;
	struct ether64_t details;
	
	size_t length;
	struct aliquid_generator_t generated [];
};

#endif
