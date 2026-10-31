#ifndef BacyStrategy_H
#define BacyStrategy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyError.d.h"

#include "BacyStrategy.d.h"






typedef struct BacyStrategy_get_file_path_result {union { BacyError* err;}; bool is_ok;} BacyStrategy_get_file_path_result;
BacyStrategy_get_file_path_result BacyStrategy_get_file_path(DiplomatStringView path, OptionI64 crc, bool no_hash, bool to_lower, DiplomatWrite* write);

void BacyStrategy_destroy(BacyStrategy* self);





#endif // BacyStrategy_H
