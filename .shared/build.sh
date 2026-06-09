#!/usr/bin/env sh
set -eu
cc -std=c11 -fPIC -DINFINITUM_SHARED_BUILD=1 -I. \
  -shared novus.c tempus.c canor.c aether.c envoy.c infinitum_gui.c infinitum.c \
  -o libinfinitum.shared.so