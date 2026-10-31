#ifndef BacyCrc64_H
#define BacyCrc64_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyError.d.h"

#include "BacyCrc64.d.h"






uint64_t BacyCrc64_compute_bytes(DiplomatU8View data);

typedef struct BacyCrc64_compute_streaming_result {union {uint64_t ok; BacyError* err;}; bool is_ok;} BacyCrc64_compute_streaming_result;
BacyCrc64_compute_streaming_result BacyCrc64_compute_streaming(DiplomatStringView path, size_t buffer_size);

typedef struct BacyCrc64_compare_result {union { BacyError* err;}; bool is_ok;} BacyCrc64_compare_result;
BacyCrc64_compare_result BacyCrc64_compare(DiplomatStringView path, uint64_t expected_crc);

void BacyCrc64_destroy(BacyCrc64* self);





#endif // BacyCrc64_H
