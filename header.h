#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Value_Tag {
  value_val1,
  value_val2,
};

struct Value {
  enum Value_Tag tag;
  union {
    struct {
      uint8_t val1;
    };
  };
};

struct Example {
  uint32_t id;
  struct Value value;
};

uint8_t *example_write(const struct Example *obj, uint16_t *out_size);

struct Example *example_from_bytes(const uint8_t *data, uintptr_t len);

void example_free(struct Example *obj);
