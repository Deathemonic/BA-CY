#ifndef BacyCrcResult_D_H
#define BacyCrcResult_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BacyCrcResult {
  uint32_t value;
} BacyCrcResult;

typedef struct BacyCrcResult_option {union { BacyCrcResult ok; }; bool is_ok; } BacyCrcResult_option;



#endif // BacyCrcResult_D_H
