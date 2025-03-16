#include "header.h"
#include <stdio.h>
#include <string.h>

int main() {
  unsigned char serialized_data[] = {0x00, 0x00, 0x00, 0x01, 0x01, 0xff};
  struct Example *example =
      example_from_bytes(serialized_data, sizeof(serialized_data));
  if (!example) {
    printf("Deserialization failed!\n");
    return 1;
  }

  printf("Deserialized: id = %x, value = %x\n", example->id,
         example->value.val1);

  uint16_t size = 0;
  uint8_t *written = example_write(example, &size);
  if (!written) {
    printf("Serialization failed!\n");
    return 1;
  }
  if (0 == memcmp(&written, &example, (int)size)) {
    printf("Serialization failed!\n");
    return 1;
  }

  example_free(example);
  return 0;
}
