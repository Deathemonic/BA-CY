#ifndef BacyMersenneTwister_H
#define BacyMersenneTwister_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyError.d.h"

#include "BacyMersenneTwister.d.h"






BacyMersenneTwister* BacyMersenneTwister_new(uint32_t seed);

uint32_t BacyMersenneTwister_next_value(BacyMersenneTwister* self);

uint32_t BacyMersenneTwister_next_u32(BacyMersenneTwister* self);

uint64_t BacyMersenneTwister_next_u64(BacyMersenneTwister* self);

void BacyMersenneTwister_next_bytes(BacyMersenneTwister* self, DiplomatU8ViewMut output);

float BacyMersenneTwister_next_f32(BacyMersenneTwister* self);

double BacyMersenneTwister_next_f64(BacyMersenneTwister* self);

typedef struct BacyMersenneTwister_next_range_result {union {int32_t ok; BacyError* err;}; bool is_ok;} BacyMersenneTwister_next_range_result;
BacyMersenneTwister_next_range_result BacyMersenneTwister_next_range(BacyMersenneTwister* self, int32_t min, int32_t max);

void BacyMersenneTwister_destroy(BacyMersenneTwister* self);





#endif // BacyMersenneTwister_H
