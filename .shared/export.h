/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#ifndef INFINITUM_EXPORT_H
#define INFINITUM_EXPORT_H

#if defined(_WIN32) || defined(__CYGWIN__)
  #if defined(INFINITUM_SHARED_BUILD)
    #define INF_SHARED_API __declspec(dllexport)
  #else
    #define INF_SHARED_API __declspec(dllimport)
  #endif
#else
  #if defined(__GNUC__) || defined(__clang__)
    #define INF_SHARED_API __attribute__((visibility("default")))
  #else
    #define INF_SHARED_API
  #endif
#endif

#ifdef __cplusplus
  #define INF_EXTERN_C extern "C"
#else
  #define INF_EXTERN_C extern
#endif

#define INF_API INF_EXTERN_C INF_SHARED_API

#endif