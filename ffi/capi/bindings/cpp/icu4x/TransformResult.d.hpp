#ifndef icu4x_TransformResult_D_HPP
#define icu4x_TransformResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum TransformResult {
      TransformResult_Modified = 0,
      TransformResult_Unmodified = 1,
    };

    typedef struct TransformResult_option {union { TransformResult ok; }; bool is_ok; } TransformResult_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `TransformResult`](https://docs.rs/icu/2.0.0/icu/locale/enum.TransformResult.html) for more information.
 */
class TransformResult {
public:
  enum Value {
    Modified = 0,
    Unmodified = 1,
  };

  TransformResult(): value(Value::Unmodified) {}

  // Implicit conversions between enum and ::Value
  constexpr TransformResult(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::TransformResult AsFFI() const;
  inline static icu4x::TransformResult FromFFI(icu4x::capi::TransformResult c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_TransformResult_D_HPP
