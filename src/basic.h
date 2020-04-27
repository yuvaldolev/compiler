#pragma once

#include <float.h>
#include <limits.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define function static
#define global static
#define local_persist static

#define _GLUE(a, b) a##b
#define GLUE(a, b) GLUE_(a, b)

#define _STRINGIFY(a) #a
#define STRINGIFY(a) _STRINGIFY(a)

#define ARRAY_COUNT(arr) (sizeof(arr) / sizeof((arr)[0]))

#define OFFSET_OF(type, member) ((umm)(&(((type*)0)->member)))

typedef int8_t s8;
typedef int16_t s16;
typedef int32_t s32;
typedef int64_t s64;

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef s8 b8;
typedef s32 b32;
typedef s64 b64;

typedef float f32;
typedef double f64;

typedef intptr_t smm;
typedef uintptr_t umm;

