#include <string.h>
#include "aether.h"

static void aether_fill_channel(struct novus8_t *channel, uint8_t v) {
    novus8_zero(channel);
    channel->data.lower = v;
    channel->data.upper = v;
    channel->settings.minimum.lower = 0u;
    channel->settings.minimum.upper = 0u;
    channel->settings.nominal.lower = v;
    channel->settings.nominal.upper = v;
    channel->settings.maximum.lower = 255u;
    channel->settings.maximum.upper = 255u;
}

void aether_color_init_rgba8(struct aether_color_t *color, uint8_t red, uint8_t green, uint8_t blue, uint8_t alpha) {
    if (color == NULL) {
        return;
    }
    aether_fill_channel(&color->red, red);
    aether_fill_channel(&color->green, green);
    aether_fill_channel(&color->blue, blue);
    aether_fill_channel(&color->alpha, alpha);
}

void aether_init(struct aether_t *value) {
    if (value == NULL) {
        return;
    }
    memset(value, 0, sizeof(*value));
    aether_color_init_rgba8(&value->color, 0u, 0u, 0u, 255u);
    canor8_init(&value->coarse);
    canor16_init(&value->medium);
    canor32_init(&value->fine);
    canor64_init(&value->details);
}

void aether_free(struct aether_t *value) {
    if (value == NULL) {
        return;
    }
    canor8_free(&value->coarse);
    canor16_free(&value->medium);
    canor32_free(&value->fine);
    canor64_free(&value->details);
}