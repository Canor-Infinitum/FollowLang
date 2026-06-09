/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_H
#define INFINITUM_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include <shared/novus32.h>
#include <shared/aether.h>

/* ---------- versioning ---------- */

#define CANOR_GUI_ABI_VERSION 1u

/* ---------- scalar & geometry ---------- */

typedef struct novus32_t gui_scalar_t;

struct gui_vec2_t {
    gui_scalar_t x;
    gui_scalar_t y;
};

struct gui_rect_t {
    gui_scalar_t x;
    gui_scalar_t y;
    gui_scalar_t width;
    gui_scalar_t height;
};

struct gui_edges_t {
    gui_scalar_t left;
    gui_scalar_t top;
    gui_scalar_t right;
    gui_scalar_t bottom;
};

struct gui_radius_t {
    gui_scalar_t top_left;
    gui_scalar_t top_right;
    gui_scalar_t bottom_right;
    gui_scalar_t bottom_left;
};

/* ---------- ids ---------- */

typedef uint64_t gui_id_t;
typedef uint64_t gui_resource_id_t;
typedef uint64_t gui_string_id_t;
typedef uint64_t gui_action_id_t;

/* ---------- enums ---------- */

enum gui_element_kind {
    GUI_ELEMENT_NONE = 0,
    GUI_ELEMENT_WINDOW,
    GUI_ELEMENT_PANEL,
    GUI_ELEMENT_LABEL,
    GUI_ELEMENT_BUTTON,
    GUI_ELEMENT_IMAGE,
    GUI_ELEMENT_INPUT,
    GUI_ELEMENT_SPLITTER,
    GUI_ELEMENT_MENU,
    GUI_ELEMENT_MENU_ITEM,
    GUI_ELEMENT_LIST,
    GUI_ELEMENT_TREE,
    GUI_ELEMENT_CANVAS,
    GUI_ELEMENT_VIEWPORT,
    GUI_ELEMENT_CUSTOM
};

enum gui_layout_kind {
    GUI_LAYOUT_NONE = 0,
    GUI_LAYOUT_ABSOLUTE,
    GUI_LAYOUT_DOCK,
    GUI_LAYOUT_FLOW,
    GUI_LAYOUT_GRID,
    GUI_LAYOUT_STACK
};

enum gui_dock_mode {
    GUI_DOCK_NONE = 0,
    GUI_DOCK_TOP,
    GUI_DOCK_BOTTOM,
    GUI_DOCK_LEFT,
    GUI_DOCK_RIGHT,
    GUI_DOCK_FILL
};

enum gui_visibility {
    GUI_VISIBILITY_VISIBLE = 0,
    GUI_VISIBILITY_HIDDEN,
    GUI_VISIBILITY_COLLAPSED
};

enum gui_text_align {
    GUI_TEXT_ALIGN_START = 0,
    GUI_TEXT_ALIGN_CENTER,
    GUI_TEXT_ALIGN_END
};

enum gui_overflow_mode {
    GUI_OVERFLOW_VISIBLE = 0,
    GUI_OVERFLOW_HIDDEN,
    GUI_OVERFLOW_SCROLL
};

/* ---------- flags ---------- */

enum gui_element_flags {
    GUI_FLAG_ENABLED        = 1u << 0,
    GUI_FLAG_FOCUSABLE      = 1u << 1,
    GUI_FLAG_DRAGGABLE      = 1u << 2,
    GUI_FLAG_RESIZABLE      = 1u << 3,
    GUI_FLAG_SELECTABLE     = 1u << 4,
    GUI_FLAG_HOVERABLE      = 1u << 5,
    GUI_FLAG_CLICKABLE      = 1u << 6,
    GUI_FLAG_CLIP_CHILDREN  = 1u << 7,
    GUI_FLAG_MODAL          = 1u << 8,
    GUI_FLAG_ACTIVE         = 1u << 9,
    GUI_FLAG_HIGHLIGHTED    = 1u << 10,
    GUI_FLAG_CONTAINER      = 1u << 11
};

/* ---------- style ---------- */

struct gui_border_t {
    gui_scalar_t width;
    struct gui_radius_t radius;
    struct aether_color_t color;
};

struct gui_shadow_t {
    bool enabled;
    uint8_t reserved8[7];
    struct gui_vec2_t offset;
    gui_scalar_t blur;
    struct aether_color_t color;
};

struct gui_style_t {
    struct aether_color_t foreground;
    struct aether_color_t background;
    struct aether_color_t accent;
    struct gui_border_t border;
    struct gui_shadow_t shadow;

    gui_scalar_t opacity;
    struct gui_edges_t margin;
    struct gui_edges_t padding;

    uint32_t z_index;
    uint32_t reserved32;
};

/* ---------- layout ---------- */

struct gui_layout_t {
    uint32_t layout_kind;    /* enum gui_layout_kind */
    uint32_t dock_mode;      /* enum gui_dock_mode */

    struct gui_rect_t rect;
    struct gui_vec2_t min_size;
    struct gui_vec2_t max_size;
    struct gui_vec2_t preferred_size;

    uint32_t row;
    uint32_t column;
    uint32_t row_span;
    uint32_t column_span;

    uint32_t visibility;     /* enum gui_visibility */
    uint32_t overflow_mode;  /* enum gui_overflow_mode */
};

/* ---------- payload blocks ---------- */

struct gui_window_payload_t {
    gui_string_id_t title;
    uint32_t can_minimize;
    uint32_t can_maximize;
    uint32_t can_close;
    uint32_t reserved32;
};

struct gui_label_payload_t {
    gui_string_id_t text;
    gui_string_id_t font_family;
    uint32_t font_size;
    uint32_t align; /* enum gui_text_align */
};

struct gui_button_payload_t {
    gui_string_id_t text;
    gui_resource_id_t icon;
    gui_action_id_t action;
    uint32_t reserved32;
};

struct gui_image_payload_t {
    gui_resource_id_t resource;
    uint32_t stretch;
    uint32_t proportional;
    uint32_t center;
    uint32_t reserved32;
};

struct gui_splitter_payload_t {
    uint32_t vertical;
    uint32_t thickness;
    uint32_t min_first;
    uint32_t min_second;
};

struct gui_input_payload_t {
    gui_string_id_t value;
    gui_string_id_t placeholder;
    uint32_t read_only;
    uint32_t reserved32;
};

struct gui_menu_item_payload_t {
    gui_string_id_t text;
    gui_action_id_t action;
    uint32_t checked;
    uint32_t disabled;
};

struct gui_custom_payload_t {
    uint64_t type_id;
    uint64_t data_id;
};

/* ---------- base element ---------- */

union gui_payload_t {
    struct gui_window_payload_t window;
    struct gui_label_payload_t label;
    struct gui_button_payload_t button;
    struct gui_image_payload_t image;
    struct gui_splitter_payload_t splitter;
    struct gui_input_payload_t input;
    struct gui_menu_item_payload_t menu_item;
    struct gui_custom_payload_t custom;
};

struct gui_element_t {
    uint32_t abi_version;
    uint32_t kind;           /* enum gui_element_kind */

    gui_id_t id;
    gui_id_t parent_id;
    gui_id_t first_child_id;
    gui_id_t next_sibling_id;

    uint64_t flags;          /* enum gui_element_flags bitmask */

    struct gui_layout_t layout;
    struct gui_style_t style;
    union gui_payload_t payload;
};

/* ---------- action / event transport ---------- */

enum gui_event_kind {
    GUI_EVENT_NONE = 0,
    GUI_EVENT_CREATE,
    GUI_EVENT_DESTROY,
    GUI_EVENT_SHOW,
    GUI_EVENT_HIDE,
    GUI_EVENT_FOCUS,
    GUI_EVENT_BLUR,
    GUI_EVENT_HOVER_ENTER,
    GUI_EVENT_HOVER_LEAVE,
    GUI_EVENT_CLICK,
    GUI_EVENT_DOUBLE_CLICK,
    GUI_EVENT_MOUSE_DOWN,
    GUI_EVENT_MOUSE_UP,
    GUI_EVENT_MOUSE_MOVE,
    GUI_EVENT_KEY_DOWN,
    GUI_EVENT_KEY_UP,
    GUI_EVENT_INPUT,
    GUI_EVENT_RESIZE,
    GUI_EVENT_MOVE,
    GUI_EVENT_CUSTOM
};

struct gui_event_t {
    uint32_t kind;    /* enum gui_event_kind */
    uint32_t reserved32;

    gui_id_t target_id;
    gui_action_id_t action_id;

    int32_t x;
    int32_t y;
    int32_t dx;
    int32_t dy;

    uint32_t key_code;
    uint32_t modifiers;

    uint64_t data0;
    uint64_t data1;
};

#ifdef __cplusplus
}
#endif

#endif