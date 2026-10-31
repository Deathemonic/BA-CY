#ifndef BacyCrc_H
#define BacyCrc_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyCrcResult.d.h"
#include "BacyError.d.h"

#include "BacyCrc.d.h"






uint32_t BacyCrc_compute_bytes(DiplomatU8View data);

uint32_t BacyCrc_compute_bytes_with_suffix(DiplomatU8View data, DiplomatU8View suffix);

typedef struct BacyCrc_compute_streaming_result {union {uint32_t ok; BacyError* err;}; bool is_ok;} BacyCrc_compute_streaming_result;
BacyCrc_compute_streaming_result BacyCrc_compute_streaming(DiplomatStringView path, size_t buffer_size);

typedef struct BacyCrc_compare_result {union { BacyError* err;}; bool is_ok;} BacyCrc_compare_result;
BacyCrc_compare_result BacyCrc_compare(DiplomatStringView path, uint32_t expected_crc);

uint32_t BacyCrc_pad_zeros(uint32_t value);

BacyCrcResult BacyCrc_evaluate(DiplomatU8View data);

void BacyCrc_destroy(BacyCrc* self);





#endif // BacyCrc_H
