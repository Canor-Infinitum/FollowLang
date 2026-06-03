/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef FOLLOWLANG_MANEUVER_H
#define FOLLOWLANG_MANEUVER_H

#include <flow.h>
#include <action.h>

#include <infinitum/novus.h>
#include <infinitum/novus8.h>
#include <infinitum/novus16.h>
#include <infinitum/novus32.h>
#include <infinitum/novus64.h>

#include <infinitum/tempus.h>

struct paradox_t {
	struct flow_t flow;
	struct action_t action;
};

struct paradoxes_t {
	struct flows_t flows;
	struct actions_t actions;
};

struct maneuver8_t {
	struct maneuver_t *parent;
	struct maneuver_t *left;
	struct maneuver_t *right;
	
	struct paradoxes_t results;
	
	struct paradoxes_t (*updater)(struct novus8_t, struct tempus_t);
	
	/* Set update_rate > 0 for dynamic systems: Remember timeout! */
	size_t update_rate;
	size_t update_timer;
	size_t timeout;
	
	/* These are called whenever the results are updated. */
	void (*update);		// All results >= 0.
	void (*epsilon);	// Some results = 0.
	void (*error);		// All results zeroed out.
};

struct maneuver16_t {
	struct maneuver_t *parent;
	struct maneuver_t *left;
	struct maneuver_t *right;
	
	struct paradoxes_t results;
	
	struct paradoxes_t (*updater)(struct novus16_t, struct tempus_t);
	
	/* Set update_rate > 0 for dynamic systems: Remember timeout! */
	size_t update_rate;
	size_t update_timer;
	size_t timeout;
	
	/* These are called whenever the results are updated. */
	void (*update);		// All results >= 0.
	void (*epsilon);	// Some results = 0.
	void (*error);		// All results zeroed out.
};

struct maneuver32_t {
	struct maneuver_t *parent;
	struct maneuver_t *left;
	struct maneuver_t *right;
	
	struct paradoxes_t results;
	
	struct paradoxes_t (*updater)(struct novus32_t, struct tempus_t);
	
	/* Set update_rate > 0 for dynamic systems: Remember timeout! */
	size_t update_rate;
	size_t update_timer;
	size_t timeout;
	
	/* These are called whenever the results are updated. */
	void (*update);		// All results >= 0.
	void (*epsilon);	// Some results = 0.
	void (*error);		// All results zeroed out.
};

struct maneuver64_t {
	struct maneuver_t *parent;
	struct maneuver_t *left;
	struct maneuver_t *right;
	
	struct paradoxes_t results;
	
	struct paradoxes_t (*updater)(struct novus64_t, struct tempus_t);
	
	/* Set update_rate > 0 for dynamic systems: Remember timeout! */
	size_t update_rate;
	size_t update_timer;
	size_t timeout;
	
	/* These are called whenever the results are updated. */
	void (*update);		// All results >= 0.
	void (*epsilon);	// Some results = 0.
	void (*error);		// All results zeroed out.
};

struct maneuver_t {
	struct maneuver_t *parent;
	struct maneuver_t *left;
	struct maneuver_t *right;
	
	struct paradoxes_t results;
	
	struct paradoxes_t (*updater)(struct novus_t, struct tempus_t);
	
	/* Set update_rate > 0 for dynamic systems: Remember timeout! */
	size_t update_rate;
	size_t update_timer;
	size_t timeout;
	
	/* These are called whenever the results are updated. */
	void (*update);		// All results >= 0.
	void (*epsilon);	// Some results = 0.
	void (*error);		// All results zeroed out.
};

#endif
