#ifndef BacySha_H
#define BacySha_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyBytes.d.h"
#include "BacyError.d.h"

#include "BacySha.d.h"






BacyBytes* BacySha_compute(DiplomatU8View source);

BacyBytes* BacySha_compute_str(DiplomatStringView source);

BacyBytes* BacySha_compute_sha1(DiplomatU8View source);

typedef struct BacySha_compute_sha1_hex_result {union { BacyError* err;}; bool is_ok;} BacySha_compute_sha1_hex_result;
BacySha_compute_sha1_hex_result BacySha_compute_sha1_hex(DiplomatU8View source, DiplomatWrite* write);

typedef struct BacySha_compute_streaming_result {union { BacyError* err;}; bool is_ok;} BacySha_compute_streaming_result;
BacySha_compute_streaming_result BacySha_compute_streaming(DiplomatStringView path, size_t buffer_size, DiplomatWrite* write);

void BacySha_destroy(BacySha* self);





#endif // BacySha_H
