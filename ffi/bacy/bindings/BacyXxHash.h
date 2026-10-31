#ifndef BacyXxHash_H
#define BacyXxHash_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BacyXxHash.d.h"






uint32_t BacyXxHash_calculate_hash(DiplomatU8View data);

uint32_t BacyXxHash_calculate_hash_str(DiplomatStringView source);

uint64_t BacyXxHash_calculate_hash64(DiplomatU8View data);

uint64_t BacyXxHash_calculate_hash64_str(DiplomatStringView source);

void BacyXxHash_destroy(BacyXxHash* self);





#endif // BacyXxHash_H
