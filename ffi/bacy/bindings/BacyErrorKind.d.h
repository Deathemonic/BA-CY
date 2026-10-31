#ifndef BacyErrorKind_D_H
#define BacyErrorKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum BacyErrorKind {
  BacyErrorKind_InvalidArgument = 0,
  BacyErrorKind_Io = 1,
  BacyErrorKind_InvalidPath = 2,
  BacyErrorKind_Crc32Mismatch = 3,
  BacyErrorKind_Crc64Mismatch = 4,
  BacyErrorKind_Base64Decode = 5,
  BacyErrorKind_Utf16 = 6,
  BacyErrorKind_StringConversion = 7,
  BacyErrorKind_AesDecrypt = 8,
  BacyErrorKind_Write = 9,
} BacyErrorKind;

typedef struct BacyErrorKind_option {union { BacyErrorKind ok; }; bool is_ok; } BacyErrorKind_option;



#endif // BacyErrorKind_D_H
