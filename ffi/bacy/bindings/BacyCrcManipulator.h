#ifndef BacyCrcManipulator_H
#define BacyCrcManipulator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BacyError.d.h"

#include "BacyCrcManipulator.d.h"






BacyCrcManipulator* BacyCrcManipulator_new(DiplomatStringView path);

typedef struct BacyCrcManipulator_forge_crc_result {union { BacyError* err;}; bool is_ok;} BacyCrcManipulator_forge_crc_result;
BacyCrcManipulator_forge_crc_result BacyCrcManipulator_forge_crc(const BacyCrcManipulator* self, uint32_t target_crc);

typedef struct BacyCrcManipulator_match_file_result {union { BacyError* err;}; bool is_ok;} BacyCrcManipulator_match_file_result;
BacyCrcManipulator_match_file_result BacyCrcManipulator_match_file(const BacyCrcManipulator* self, DiplomatStringView target_path);

void BacyCrcManipulator_destroy(BacyCrcManipulator* self);





#endif // BacyCrcManipulator_H
